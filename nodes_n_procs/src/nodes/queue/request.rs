use core::array;
use core::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use core::iter::FromIterator;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use either::Either;
use heapless::{FnvIndexMap, FnvIndexSet};
use crate::base::node::*;

#[derive(PartialEq)]
pub enum Request<TReq, TAns> {
    Question(TReq),
    Processing,
    Answer(TAns),
}

pub type RequestSituation = Request<(), ()>;

impl<TReq, TAns> Request<TReq, TAns> {
    pub fn has_answer(&self) -> bool{
        match self{
            Request::Question(_) | Request::Processing => {false}
            Request::Answer(_) => {true}
        }
    }
}


pub type RequestNode<TReq, TAns> = Node<Option<Request<TReq, TAns>>>;
pub type RequestNRef<'a, TReq, TAns> = <RequestNode<TReq, TAns> as TryDeref>::TRef<'a>;
pub type RequestNMut<'a, TReq, TAns> = <RequestNode<TReq, TAns> as TryDerefMut>::TMut<'a>;

pub mod request_queue {
    use super::*;
    use core::array;
    use core::iter::FromIterator;
    use core::marker::PhantomData;
    use core::mem::{ManuallyDrop, MaybeUninit};
    use core::ops::Deref;
    use heapless::{FnvIndexMap, FnvIndexSet};
    use nnp_base::runner::TryDeref;
    use crate::base::extensions::used_in::UsedInTrait;
    use crate::base::node::Node;
    use crate::queue::request::Request;


    use self::request_queue_internals::RequestQueue;

    pub type RequestQueueNode<TReq, TAns, const QUEUE_SIZE: usize> = Node<RequestQueue<TReq, TAns, QUEUE_SIZE>>;
    pub type RequestQueueNRef<'a, TReq, TAns, const QUEUE_SIZE: usize> = <Node<RequestQueue<TReq, TAns, QUEUE_SIZE>> as TryDeref>::TRef<'a>;
    pub type RequestQueueNMut<'a, TReq, TAns, const QUEUE_SIZE: usize> = <Node<RequestQueue<TReq, TAns, QUEUE_SIZE>> as TryDerefMut>::TMut<'a>;
    pub type RequestIdentifier = u32;

    mod request_queue_internals {
        use core::mem::MaybeUninit;
        use heapless::FnvIndexSet;
        use super::*;
        pub struct RequestQueue<TReq, TAns, const QUEUE_SIZE: usize> {
            pub(super) last_identifier:    RequestIdentifier,
            pub(super) id_situations_data: IdSituationKeeper<QUEUE_SIZE>,
            pub(super) loc_info:           LocationInfoKeeper<QUEUE_SIZE>,
            pub(super) queue_data:         [RequestQueueDataCell<TReq, TAns>; QUEUE_SIZE],
        }

        pub(super) struct IdSituationKeeper<const SIZE: usize> {
            pub(super) question_ids:   FnvIndexSet<RequestIdentifier, SIZE>,
            pub(super) processing_ids: FnvIndexSet<RequestIdentifier, SIZE>,
            pub(super) answer_ids:     FnvIndexSet<RequestIdentifier, SIZE>,
        }

        #[derive(Debug)]
        pub enum SituationChangeError {
            OldSituationIncorrect,
            AlreadyWasNewSituation
        }

        impl<const SIZE: usize> IdSituationKeeper<SIZE> {
            pub(super) fn set_situation(
                &mut self,
                req_id: RequestIdentifier,
                old_situation: Option<RequestSituation>,
                new_situation: Option<RequestSituation>,
            ) -> Result<(), SituationChangeError> { unsafe {
                if new_situation != old_situation {
                    if let Some(old_situation) = old_situation {
                        let has_removed =
                        self
                        .get_set_mut(&old_situation)
                        .remove(&req_id);

                        if !has_removed { return Err(SituationChangeError::OldSituationIncorrect) }
                    }
                    if let Some(new_situation) = new_situation {
                        let has_inserted =
                        self
                        .get_set_mut(&new_situation)
                        .insert(req_id)
                        .unwrap_unchecked();
                        if !has_inserted { return Err(SituationChangeError::AlreadyWasNewSituation); }
                    }
                }
                Ok(())
            } }
            pub(super) fn get_situation(&self, req_id: RequestIdentifier) -> Option<RequestSituation> {
                if self.question_ids.contains(&req_id) {
                    Some(RequestSituation::Question(()))
                } else if self.processing_ids.contains(&req_id) {
                    Some(RequestSituation::Processing)
                } else if self.answer_ids.contains(&req_id) {
                    Some(RequestSituation::Answer(()))
                } else {
                    None
                }
            }
            pub(super) unsafe fn set_situation_unchecked(
                &mut self,
                req_id:         RequestIdentifier,
                old_situation:  Option<RequestSituation>,
                new_situation:  Option<RequestSituation>,
            ) {
                unsafe {
                    if new_situation != old_situation {
                        if let Some(old_situation) = old_situation {
                            self.get_set_mut(&old_situation).remove(&req_id);
                        }
                        if let Some(new_situation) = new_situation {
                            self.get_set_mut(&new_situation)
                            .insert(req_id)
                            .unwrap_unchecked();
                        }
                    }
                }
            }

            fn get_set_mut(&mut self, situation: &RequestSituation) -> &mut FnvIndexSet<RequestIdentifier,
                SIZE> {
                match situation {
                    RequestSituation::Question(_)   => &mut self.question_ids,
                    RequestSituation::Processing    => &mut self.processing_ids,
                    RequestSituation::Answer(_)     => &mut self.answer_ids
                }
            }
        }

        pub(super) struct LocationInfoKeeper<const SIZE: usize> {
            pub(super) id_to_location: FnvIndexMap<RequestIdentifier, usize, SIZE>,
            pub(super) open_locations: FnvIndexSet<usize, SIZE>,
        }

        impl<const SIZE: usize> LocationInfoKeeper<SIZE> {
            pub(super) fn get_open_loc(&mut self, req_id: RequestIdentifier) -> Option<usize> {
                let loc = *self.open_locations.first()?;
                self.id_to_location.insert(req_id, loc).unwrap();
                self.open_locations.remove(&loc);

                Some(loc)
            }
            pub(super) fn clear_loc(&mut self, req_id: RequestIdentifier) -> bool {
                if let Some(loc) = self.id_to_location.remove(&req_id) {
                    self.open_locations.insert(loc).unwrap();
                    true
                } else {
                    false
                }
            }
        }

        pub(super) union RequestQueueDataCellContent<TReq, TAns> {
            pub(super) question:   ManuallyDrop<TReq>,
            pub(super) answer:     ManuallyDrop<TAns>,
        }

        pub(super) type RequestQueueDataCell<TReq, TAns> = MaybeUninit<RequestQueueDataCellContent<TReq, TAns>>;

        pub(super) struct RequestQueueReqHeader {
            pub(super) location:   usize,
            pub(super) situation:  RequestSituation,
            pub(super) id:         RequestIdentifier,
        }

        impl<TReq, TAns, const QUEUE_SIZE: usize> RequestQueue<TReq, TAns, QUEUE_SIZE> {
            pub fn new() -> Self {
                Self {
                    last_identifier: 0,
                    id_situations_data: IdSituationKeeper {
                        question_ids: Default::default(),
                        processing_ids: Default::default(),
                        answer_ids: Default::default(),
                    },
                    loc_info: LocationInfoKeeper {
                        id_to_location: Default::default(),
                        open_locations: FnvIndexSet::from_iter(0..QUEUE_SIZE),
                    },
                    queue_data: array::from_fn(|_i| MaybeUninit::uninit()),
                }
            }

            pub(super) fn get_req_header(&self, req_id: RequestIdentifier) -> Option<RequestQueueReqHeader> {
                let situation = self.id_situations_data.get_situation(req_id)?;

                let location =
                    self
                    .loc_info.id_to_location
                    .get(&req_id)
                    .used_in(|index| unsafe { index.unwrap_unchecked() })
                    .clone();

                Some(
                    RequestQueueReqHeader {
                        location,
                        situation,
                        id: req_id
                    }
                )
            }
            pub(super) fn get_req(&self, req_id: RequestIdentifier) -> Option<Request<&TReq, &TAns>> {
                let header =
                self.get_req_header(req_id)?;

                unsafe {
                    let data_cell = self.queue_data.get_unchecked(header.location);
                    match &header.situation {
                        RequestSituation::Question(_) =>
                            Request::Question(data_cell.assume_init_ref().question.deref()),
                        RequestSituation::Processing =>
                            Request::Processing,
                        RequestSituation::Answer(_) =>
                            Request::Answer(data_cell.assume_init_ref().answer.deref())
                    }
                }
                .used_in(Some)
            }
        }
    }
    
    use request_queue_internals::*;
    pub mod request_queue_client{
        use super::*;

        impl<TReq, TAns, const QUEUE_SIZE: usize> RequestQueue<TReq, TAns, QUEUE_SIZE>{
            pub fn make_client_endpoint(&self) -> RequestQueueClientEndpoint<TReq, TAns> {
                RequestQueueClientEndpoint::new()
            }
        } 

        pub struct RequestQueueClientEndpoint<TReq, TAns>{
            current_req_id: Option<RequestIdentifier> ,
            io_phantom:     PhantomData<(TReq, TAns)>
        }

        impl<TReq, TAns> RequestQueueClientEndpoint<TReq, TAns> {
            pub fn new() -> Self { 
                Self {
                    current_req_id: None,
                    io_phantom: PhantomData{} 
                } 
            }
            pub fn has_request(&self) -> bool {self.current_req_id.is_some()}
            pub fn current_request_id(&self) -> &Option<RequestIdentifier>{&self.current_req_id}
            pub fn current_request_id_mut(&mut self) -> &mut Option<RequestIdentifier>{&mut self.current_req_id}
        }

        pub struct RequestQueueClientConnection<'a, TReq, TAns, TReqQueueDerefMut, const QUEUE_SIZE: usize>
            where TReqQueueDerefMut: DerefMut<Target=RequestQueue<TReq, TAns, QUEUE_SIZE>> + 'a
        {
            queue:    TReqQueueDerefMut,
            endpoint: &'a mut RequestQueueClientEndpoint<TReq, TAns>    
        }

        impl<TReq, TAns> RequestQueueClientEndpoint<TReq, TAns> {
            pub fn connect
                <'a, TQueueDerefMut, const QUEUE_SIZE: usize>
                (&'a mut self, queue: TQueueDerefMut) 
                -> RequestQueueClientConnection<'a,  TReq, TAns, TQueueDerefMut, QUEUE_SIZE>
                where TQueueDerefMut: DerefMut<Target=RequestQueue<TReq, TAns, QUEUE_SIZE>> + 'a 
            {
                RequestQueueClientConnection{
                    queue: queue, endpoint: self
                }   
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum QueueRequestQueryError{
            RequestNotFound,
            NoRequest,
        }

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum QueueRequestPostError{
            AlreadyHasRequest,
            QueueFull,
        }

        impl<'a,  TReq, TAns, TQueueDerefMut, const QUEUE_SIZE: usize> 
            RequestQueueClientConnection<'a,  TReq, TAns,TQueueDerefMut, QUEUE_SIZE> 
            where TQueueDerefMut: DerefMut<Target=RequestQueue<TReq, TAns, QUEUE_SIZE>> + 'a 
        {
            pub fn try_post_request(&mut self, req: TReq) -> Result<(), (QueueRequestPostError, TReq)> {
                if self.endpoint.has_request(){
                    return Err((QueueRequestPostError::AlreadyHasRequest, req))
                }

                let new_req_id = self.queue.last_identifier + 1;

                let slot = {
                    let location = match self.queue.loc_info.get_open_loc(new_req_id).ok_or(()) {
                        Ok(loc) => loc,
                        Err(_) => { return Err((QueueRequestPostError::QueueFull, req)) }
                    };

                    self.queue.queue_data.get_mut(location).unwrap()
                };

                *slot = MaybeUninit::new(RequestQueueDataCellContent { question: ManuallyDrop::new(req) });

                self.queue.id_situations_data.set_situation(
                    new_req_id,
                    None,
                    Some(RequestSituation::Question(()))
                );

                self.queue.last_identifier = new_req_id;
                self.endpoint.current_req_id = Some(new_req_id);

                Ok(())
            }

            pub fn try_take_answer(&mut self) -> Result<Option<TAns>, QueueRequestQueryError> {
                let req_id = self.endpoint.current_req_id.ok_or(QueueRequestQueryError::NoRequest)?;
                let req_header = self.queue.get_req_header(req_id).ok_or(QueueRequestQueryError::RequestNotFound)?;

                if req_header.situation == Request::Answer(()) {
                    let value = unsafe {
                        let value =
                            self.queue.queue_data
                            .get_unchecked(req_header.location)
                            .assume_init_read()
                            .answer
                            .used_in(ManuallyDrop::<TAns>::into_inner);

                        self.queue.id_situations_data
                        .set_situation(
                            req_id,
                            Some(RequestSituation::Answer(())),
                            None,
                        )
                        .unwrap();

                        self.queue.loc_info.clear_loc(req_id);
                        value
                    };
                    self.endpoint.current_req_id = None;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
            pub fn is_ready(&self, req_id: RequestIdentifier) -> Result<bool, QueueRequestQueryError> {
                Ok(self.queue.get_req(req_id).ok_or(QueueRequestQueryError::RequestNotFound)?.has_answer())
            }
        }
    }
    pub use request_queue_client::*;

    pub mod request_queue_server{
        pub use super::*;
        // impl RequestQueue<>
        // pub fn make_server_endpoint(&self) -> RequestQueueServerEndpoint<TReq, TAns> {
        //         RequestQueueServerEndpoint { kernel_ref: self.kernel_node.make_ref() }
        //     }
        // }

        pub struct RequestQueueServerEndpoint<TReq, TAns> {
            current_req_id: Option<RequestIdentifier>,
            io_phantom: PhantomData<(TReq, TAns)>,
        }

        impl<TReq, TAns> RequestQueueServerEndpoint<TReq, TAns> {
            pub fn new() -> Self{Self{current_req_id: None, io_phantom: PhantomData{}}}    
            pub fn has_request(&self) -> bool {self.current_req_id.is_some()}
        }


        pub struct RequestQueueServerConnection<'a, TReq, TAns, TQueueDerefMut, const QUEUE_SIZE: usize> 
            where TQueueDerefMut: DerefMut<Target=RequestQueue<TReq, TAns, QUEUE_SIZE>> + 'a
        {
            endpoint: &'a mut RequestQueueServerEndpoint<TReq, TAns>,
            queue:    TQueueDerefMut,
        }
        pub enum GetQuestionError{
            AlreadyHasQuestion,
        }

        pub enum PutAnswerError {
            NoRequestId,
            WeirdRequestId
        }

        impl<'a, TReq, TAns, TQueueDerefMut, const QUEUE_SIZE: usize> 
            RequestQueueServerConnection<'a, TReq, TAns, TQueueDerefMut, QUEUE_SIZE> 
            where TQueueDerefMut: DerefMut<Target=RequestQueue<TReq, TAns, QUEUE_SIZE>>
        {
            pub fn get_question(&mut self) -> Result<Option<TReq>, GetQuestionError> {
                if self.endpoint.has_request(){
                    return Err(GetQuestionError::AlreadyHasQuestion);
                }

                let data = 
                    self.queue
                    .id_situations_data
                    .question_ids
                    .first()
                    .cloned()
                    .map(|req_id| {
                        let req = unsafe {
                            let loc = self.queue.loc_info.id_to_location.get(&req_id).unwrap().clone();
                            let slot = self.queue.queue_data.get_unchecked_mut(loc);
                            let req = ManuallyDrop::<TReq>::into_inner(slot.assume_init_read().question);
                            req
                        };

                        self.queue.id_situations_data.set_situation(
                            req_id,
                            Some(Request::Question(())),
                            Some(Request::Processing)
                        )
                        .unwrap();

                        self.endpoint.current_req_id = Some(req_id);

                        req
                });

                Ok(data)
            }
            pub fn put_answer(&mut self, ans: TAns) -> Result<(), (PutAnswerError, TAns)>
            {
                let req_id = match self.endpoint.current_req_id{
                    Some(req_id) => req_id,
                    None         => {return Err((PutAnswerError::NoRequestId, ans));}
                };

                if !self.queue.id_situations_data.processing_ids.contains(&req_id) {
                    return Err((PutAnswerError::WeirdRequestId, ans));
                }

                let slot = {
                    let loc = self.queue.loc_info.id_to_location.get(&req_id).cloned().unwrap();
                    let slot = unsafe { self.queue.queue_data.get_mut(loc).unwrap_unchecked() };
                    slot
                };

                slot.write(
                    RequestQueueDataCellContent {
                        answer: ManuallyDrop::new(ans)
                    }
                );

                self.queue.id_situations_data
                .set_situation(req_id, Some(Request::Processing), Some(Request::Answer(())))
                .unwrap();

                Ok(())
            }
        }
    }
}
