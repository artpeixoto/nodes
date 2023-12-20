use core::array;
use core::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use core::iter::FromIterator;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Deref, DerefMut};
use either::Either;
use heapless::{FnvIndexMap, FnvIndexSet};
use base::extensions::used_in::UsedInTrait;
use base::Node;
use queue::request::Request::{Answer, Processing};
use queue::request::request_queue::RequestQueue;

#[derive(PartialEq)]
pub enum Request<TReq, TAns> {
    Question(TReq),
    Processing,
    Answer(TAns),
}

impl<TReq, TAns> Request<TReq, TAns> {
    pub fn has_answer(&self) -> bool{
        match self{
            Request::Question(_) | Request::Processing => {false}
            Request::Answer(_) => {true}
        }
    }
}


pub type RequestNode<TReq, TAns> = Node<Option<Request<TReq, TAns>>>;
pub type RequestRef<'a, TReq, TAns> = RequestNode<TReq, TAns>::TRef<'a>;
pub type RequestMut<'a, TReq, TAns> = RequestNode<TReq, TAns>::TMut<'a>;

mod request_queue {
    use core::array;
    use core::iter::FromIterator;
    use core::mem::{ManuallyDrop, MaybeUninit};
    use core::ops::Deref;
    use heapless::{FnvIndexMap, FnvIndexSet};
    use base::extensions::used_in::UsedInTrait;
    use base::Node;
    use queue::request::Request;

    pub type RequestQueueNode<TReq, TAns, const QUEUE_SIZE: usize> = Node<RequestQueue<TReq, TAns, QUEUE_SIZE>>;
    pub type RequestQueueNRef<'a, TReq, TAns, const QUEUE_SIZE: usize> = Node<RequestQueue<TReq, TAns, QUEUE_SIZE>>::TRef<'a>;
    pub type RequestQueueNMut<'a, TReq, TAns, const QUEUE_SIZE: usize> = Node<RequestQueue<TReq, TAns, QUEUE_SIZE>>::TMut<'a>;
    pub type RequestIdentifier = u32;

    mod request_queue_internals{

        struct IdSituationKeeper<const SIZE: usize> {
            question_ids: FnvIndexSet<RequestIdentifier, SIZE>,
            processing_ids: FnvIndexSet<RequestIdentifier, SIZE>,
            answer_ids: FnvIndexSet<RequestIdentifier, SIZE>,
        }

        #[derive(Debug)]
        pub enum SituationChangeError {
            OldSituationIncorrect,
            AlreadyWasNewSituation
        }

        impl<const SIZE: usize> IdSituationKeeper<SIZE> {
            fn set_situation(
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
            fn get_situation(&self, req_id: RequestIdentifier) -> Option<RequestSituation> {
                if self.question_ids.contains(&req_id) {
                    Some(Request::Question(()))
                } else if self.processing_ids.contains(&req_id) {
                    Some(Request::Processing)
                } else if self.answer_ids.contains(&req_id) {
                    Some(Request::Answer(()))
                } else {
                    None
                }
            }


            unsafe fn set_situation_unchecked(
                &mut self,
                req_id: RequestIdentifier,
                old_situation: Option<RequestSituation>,
                new_situation: Option<RequestSituation>,
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

        struct LocationInfoKeeper<const SIZE: usize> {
            id_to_location: FnvIndexMap<RequestIdentifier, usize, SIZE>,
            open_locations: FnvIndexSet<usize, SIZE>,
        }

        impl<const SIZE: usize> LocationInfoKeeper<SIZE> {
            fn get_open_loc(&mut self, req_id: RequestIdentifier) -> Option<usize> {
                let loc = *self.open_locations.first()?;
                self.id_to_location.insert(req_id, loc).unwrap();
                self.open_locations.remove(&loc);

                Some(loc)
            }
            fn clear_loc(&mut self, req_id: RequestIdentifier) -> bool {
                if let Some(loc) = self.id_to_location.remove(&req_id) {
                    self.open_locations.insert(loc).unwrap();
                    true
                } else {
                    false
                }
            }
        }

        pub struct RequestQueue<TReq, TAns, const QUEUE_SIZE: usize> {
            last_identifier: RequestIdentifier,
            id_situations_data: IdSituationKeeper<QUEUE_SIZE>,
            loc_info: LocationInfoKeeper<QUEUE_SIZE>,
            queue_data: [QueueDataCell<TReq, TAns>; QUEUE_SIZE],
        }

        union RequestUnion<TReq, TAns> {
            question: ManuallyDrop<TReq>,
            answer: ManuallyDrop<TAns>,
        }

        type RequestSituation = Request<(), ()>;
        type QueueDataCell<TReq, TAns> = MaybeUninit<RequestUnion<TReq, TAns>>;

        struct QueueRequestHeader {
            location: usize,
            situation: RequestSituation,
            id: RequestIdentifier,
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

            fn get_req_header(&self, req_id: RequestIdentifier) -> Option<QueueRequestHeader> {
                let situation = self.id_situations_data.get_situation(req_id)?;

                let location =
                self.loc_info.id_to_location
                .get(&req_id)
                .used_in(|index| unsafe { index.unwrap_unchecked() })
                .clone();

                Some(
                    QueueRequestHeader {
                        location,
                        situation,
                        id: req_id
                    }
                )
            }
            fn get_req(&self, req_id: RequestIdentifier) -> Option<Request<&TReq, &TAns>> {
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
    pub fn make_client_endpoint(&self) -> RequestQueueClientEndpoint<TReq, TAns, QUEUE_SIZE> {
        RequestQueueClientEndpoint { kernel_ref: self.kernel_node.make_ref() }
    }

    pub fn make_server_endpoint(&self) -> RequestQueueServerEndpoint<TReq, TAns, QUEUE_SIZE> {
        RequestQueueServerEndpoint { kernel_ref: self.kernel_node.make_ref() }
    }

    pub struct RequestQueueClientEndpointConn<'a, TReqQueueDeref, TReq, TAns, const QUEUE_SIZE: usize>
        where TReqQueueDeref: Deref<Target=RequestQueueNode<> >
    {
        <'a,RequestQueueKernel<TReq, TAns, QUEUE_SIZE> >
    }

    impl<'a, TReq, TAns, const QUEUE_SIZE: usize> RequestQueueClientEndpoint<'a, TReq, TAns, QUEUE_SIZE> {
        pub fn try_post_request(&mut self, req: TReq) -> Result<RequestIdentifier, TReq> {
            let mut kernel_ref = match self.kernel_ref.try_borrow_mut() {
                Ok(refl) => refl,
                Err(_) => return Err(req)
            };

            let new_req_id = kernel_ref.last_identifier + 1;

            let slot = {
                let location = match kernel_ref.loc_info.get_open_loc(new_req_id).ok_or(()) {
                    Ok(loc) => loc,
                    Err(_) => { return Err(req) }
                };

                kernel_ref.queue_data.get_mut(location).unwrap()
            };
            *slot = MaybeUninit::new(RequestUnion { question: ManuallyDrop::new(req) });


            unsafe {
                kernel_ref.id_situations_data.set_situation_unchecked(
                    new_req_id,
                    None,
                    Some(RequestSituation::Question(()))
                );
            }

            kernel_ref.last_identifier = new_req_id;
            Ok(new_req_id)
        }

        pub fn try_take_answer(&mut self, req_id: RequestIdentifier) -> Result<Option<TAns>, QueueRequestQueryError> {
            let mut kernel_ref = self.kernel_ref.try_borrow_mut()?;
            let req_header =
            kernel_ref.get_req_header(req_id).ok_or(QueueRequestQueryError::RequestNotFound)?;

            if req_header.situation == Request::Answer(()) {
                let value = unsafe {
                    let value =
                    kernel_ref.queue_data
                    .get_unchecked(req_header.location)
                    .assume_init_read()
                    .answer
                    .used_in(ManuallyDrop::<TAns>::into_inner);

                    kernel_ref.id_situations_data
                    .set_situation(
                        req_id,
                        Some(RequestSituation::Answer(())),
                        None,
                    )
                    .unwrap()
                    ;

                    kernel_ref.loc_info.clear_loc(req_id);
                    value
                };
                Ok(Some(value))
            } else {
                Ok(None)
            }
        }
        pub fn is_ready(&self, req_id: RequestIdentifier) -> Result<bool, QueueRequestQueryError> {
            let kernel_ref = self.kernel_ref.try_borrow()?;

            Ok(kernel_ref.get_req(req_id).ok_or(RequestNotFound)?.has_answer())
        }


        pub struct RequestQueueServerEndpoint<'a, TReq, TAns, const QUEUE_SIZE: usize> {
            kernel_ref: NodeRef<'a, RequestQueue<TReq, TAns, QUEUE_SIZE>>
        }
        pub enum PutAnswerError {
            WeirdRequestId
        }
        impl<'a, TReq, TAns, const QUEUE_SIZE: usize> RequestQueueServerEndpoint<'a, TReq, TAns, QUEUE_SIZE> {
            pub fn get_question(&mut self) -> Result<Option<(RequestIdentifier, TReq)>, NodeBorrowError> {
                let mut kernel_ref = self.kernel_ref.try_borrow_mut()?;
                let data = kernel_ref.id_situations_data.question_ids.first().cloned().map(|req_id| {
                    let req = unsafe {
                        let loc = kernel_ref.loc_info.id_to_location.get(&req_id).unwrap().clone();
                        let slot = kernel_ref.queue_data.get_unchecked_mut(loc);
                        let req = ManuallyDrop::<TReq>::into_inner(slot.assume_init_read().question);
                        req
                    };

                    kernel_ref.id_situations_data.set_situation(
                        req_id,
                        Some(Request::Question(())),
                        Some(Request::Processing)
                    )
                    .unwrap();

                    (req_id, req)
                });

                Ok(data)
            }
            pub fn put_answer(&mut self, req_id: RequestIdentifier, ans: TAns) -> Result<(),
                (Either<NodeBorrowError, PutAnswerError>, TAns)>
            {
                let mut kernel_ref =
                match self.kernel_ref.try_borrow_mut() {
                    Ok(kernel_ref) => kernel_ref,
                    Err(borrow_err) => {
                        return Err((Either::Left(NodeBorrowError::from(borrow_err)
                        ), ans));
                    }
                };

                if !kernel_ref.id_situations_data.processing_ids.contains(&req_id) {
                    return Err((
                        Either::Right(PutAnswerError::WeirdRequestId),
                        ans,
                    )
                    );
                }

                let slot = {
                    let loc = kernel_ref.loc_info.id_to_location.get(&req_id).cloned().unwrap();
                    let slot = unsafe { kernel_ref.queue_data.get_mut(loc).unwrap_unchecked() };
                    slot
                };

                slot.write(
                    RequestUnion {
                        answer: ManuallyDrop::new(ans)
                    }
                );

                kernel_ref.id_situations_data.set_situation(req_id, Some(Processing), Some(Answer(()))).unwrap();

                Ok(())
            }
        }
    }
}