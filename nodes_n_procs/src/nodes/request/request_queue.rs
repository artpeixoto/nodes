use super::*;
use core::array;
use core::iter::FromIterator;
use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::Deref;
use heapless::{FnvIndexMap, FnvIndexSet};
use nnp_base::runner::{TryDeref, TryDerefMut};
use crate::base::extensions::used_in::UsedInTrait;
use crate::base::node::Node;
use self::request_queue_internals::RequestQueue;
pub type RequestQueueNode<TQues, TAns, const QUEUE_SIZE: usize> = Node<RequestQueue<TQues, TAns, QUEUE_SIZE>>;
pub type RequestQueueNRef<'a, TQues, TAns, const QUEUE_SIZE: usize> = <Node<RequestQueue<TQues, TAns, QUEUE_SIZE>> as TryDeref>::TRef<'a>;
pub type RequestQueueNMut<'a, TQues, TAns, const QUEUE_SIZE: usize> = <Node<RequestQueue<TQues, TAns, QUEUE_SIZE>> as TryDerefMut>::TMut<'a>;
pub type RequestIdentifier = usize;


#[derive(PartialEq, Eq)]
enum RequestSituation {
	Question,
	Processing,
	Answer	
}
mod request_queue_internals {
	use core::mem::MaybeUninit;
	use heapless::FnvIndexSet;
	use self::request::{Request};

	use super::*;
	pub struct RequestQueue<TQues, TAns, const QUEUE_SIZE: usize> {
		pub(super) last_identifier:    RequestIdentifier,
		pub(super) id_situations_data: IdSituationKeeper<QUEUE_SIZE>,
		pub(super) loc_info:           LocationInfoKeeper<QUEUE_SIZE>,
		pub(super) queue_data:         [RequestQueueDataCell<TQues, TAns>; QUEUE_SIZE],
	}

	pub(super) struct IdSituationKeeper<const QUEUE_SIZE: usize> {
		pub(super) question_ids:   FnvIndexSet<RequestIdentifier, QUEUE_SIZE>,
		pub(super) processing_ids: FnvIndexSet<RequestIdentifier, QUEUE_SIZE>,
		pub(super) answer_ids:     FnvIndexSet<RequestIdentifier, QUEUE_SIZE>,
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
				Some(RequestSituation::Question)
			} else if self.processing_ids.contains(&req_id) {
				Some(RequestSituation::Processing)
			} else if self.answer_ids.contains(&req_id) {
				Some(RequestSituation::Answer)
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
				RequestSituation::Question   	=> &mut self.question_ids,
				RequestSituation::Processing    => &mut self.processing_ids,
				RequestSituation::Answer     	=> &mut self.answer_ids
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

	pub(super) union RequestQueueDataCellContent<TQues, TAns> {
		pub(super) question:   ManuallyDrop<TQues>,
		pub(super) answer:     ManuallyDrop<TAns>,
	}

	pub(super) type RequestQueueDataCell<TQues, TAns> = MaybeUninit<RequestQueueDataCellContent<TQues, TAns>>;

	pub(super) struct RequestQueueReqHeader {
		pub(super) location:   usize,
		pub(super) situation:  RequestSituation,
		pub(super) id:         RequestIdentifier,
	}

	impl<TQues, TAns, const QUEUE_SIZE: usize> RequestQueue<TQues, TAns, QUEUE_SIZE> {
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
		pub(super) fn get_req(&self, req_id: RequestIdentifier) -> Option<Request<&TQues, &TAns>> {
			let header =
			self.get_req_header(req_id)?;

			unsafe {
				let data_cell = self.queue_data.get_unchecked(header.location);
				match &header.situation {
					RequestSituation::Question =>
						Request::Question(data_cell.assume_init_ref().question.deref()),
					RequestSituation::Processing =>
						Request::Processing,
					RequestSituation::Answer =>
						Request::Answer(data_cell.assume_init_ref().answer.deref())
				}
			}
			.used_in(Some)
		}
	}
}

use request_queue_internals::*;

pub mod request_queue_client{
use core::ops::DerefMut;


use super::*;

	impl<TQues, TAns, const QUEUE_SIZE: usize> RequestQueue<TQues, TAns, QUEUE_SIZE>{
		pub fn make_client_endpoint(&self) -> RequestQueueClientEndpoint<TQues, TAns> {
			RequestQueueClientEndpoint::new()
		}
	} 

	pub struct RequestQueueClientEndpoint<TQues, TAns>{
		current_req_id: Option<RequestIdentifier> ,
		io_phantom:     PhantomData<(TQues, TAns)>
	}

	impl<TQues, TAns> RequestQueueClientEndpoint<TQues, TAns> {
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

	pub struct RequestQueueClientConnection<'a, TQues, TAns, TQuesQueueDerefMut, const QUEUE_SIZE: usize>
		where TQuesQueueDerefMut: DerefMut<Target=RequestQueue<TQues, TAns, QUEUE_SIZE>> + 'a
	{
		queue:    TQuesQueueDerefMut,
		endpoint: &'a mut RequestQueueClientEndpoint<TQues, TAns>    
	}

	impl<TQues, TAns> RequestQueueClientEndpoint<TQues, TAns> {
		pub fn connect
			<'a, TQueueDerefMut, const QUEUE_SIZE: usize>
			(&'a mut self, queue: TQueueDerefMut) 
			-> RequestQueueClientConnection<'a,  TQues, TAns, TQueueDerefMut, QUEUE_SIZE>
			where TQueueDerefMut: DerefMut<Target=RequestQueue<TQues, TAns, QUEUE_SIZE>> + 'a 
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

	impl<'a,  TQues, TAns, TQueueDerefMut, const QUEUE_SIZE: usize> 
		RequestQueueClientConnection<'a,  TQues, TAns,TQueueDerefMut, QUEUE_SIZE> 
		where TQueueDerefMut: DerefMut<Target=RequestQueue<TQues, TAns, QUEUE_SIZE>> + 'a 
	{
		pub fn try_post_request(&mut self, req: TQues) -> Result<(), (QueueRequestPostError, TQues)> {
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
				Some(RequestSituation::Question)
			);

			self.queue.last_identifier = new_req_id;
			self.endpoint.current_req_id = Some(new_req_id);

			Ok(())
		}

		pub fn try_take_answer(&mut self) -> Result<Option<TAns>, QueueRequestQueryError> {
			let req_id = self.endpoint.current_req_id.ok_or(QueueRequestQueryError::NoRequest)?;
			let req_header = self.queue.get_req_header(req_id).ok_or(QueueRequestQueryError::RequestNotFound)?;

			if req_header.situation == RequestSituation::Answer{
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
						Some(RequestSituation::Answer),
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
			Ok(self.queue.get_req(req_id).ok_or(QueueRequestQueryError::RequestNotFound)?.is_answer())
		}
	}
}
pub use request_queue_client::*;

pub mod request_queue_server{
	use core::ops::DerefMut;

use self::request::Request;

pub use super::*;
	impl<TQues, TAns> 
		RequestQueueServerEndpoint<TQues, TAns>
	{
		pub fn make_server_connection<TQueueDerefMut, const QUEUE_SIZE: usize>
		(
			&mut self,
			queue: TQueueDerefMut
		) 
		-> RequestQueueServerConnection<TQues, TAns, TQueueDerefMut, QUEUE_SIZE>
		where TQueueDerefMut: DerefMut<Target = RequestQueue<TQues, TAns, QUEUE_SIZE>> 
		{
			RequestQueueServerConnection{
				endpoint: self,
				queue: queue
			}		
		}
	}

	pub struct RequestQueueServerEndpoint<TQues, TAns> {
		current_req_id: Option<RequestIdentifier>,
		io_phantom: PhantomData<(TQues, TAns)>,
	}

	impl<TQues, TAns> RequestQueueServerEndpoint<TQues, TAns> {
		pub fn new() -> Self{Self{current_req_id: None, io_phantom: PhantomData{}}}    
		pub fn has_request(&self) -> bool {self.current_req_id.is_some()}
	}


	pub struct RequestQueueServerConnection<'a, TQues, TAns, TQueueDerefMut, const QUEUE_SIZE: usize> 
		where TQueueDerefMut: DerefMut<Target=RequestQueue<TQues, TAns, QUEUE_SIZE>> + 'a
	{
		endpoint: &'a mut RequestQueueServerEndpoint<TQues, TAns>,
		queue:    TQueueDerefMut,
	}

	#[derive(Debug)]
	pub enum GetQuestionError{
		AlreadyHasQuestion,
	}

	#[derive(Debug)]
	pub enum PutAnswerError {
		NoRequestId,
		WeirdRequestId
	}

	impl<'a, TQues, TAns, TQueueDerefMut, const QUEUE_SIZE: usize> 
		RequestQueueServerConnection<'a, TQues, TAns, TQueueDerefMut, QUEUE_SIZE> 
		where TQueueDerefMut: DerefMut<Target=RequestQueue<TQues, TAns, QUEUE_SIZE>>
	{

		pub fn get_question(&mut self) -> Result<Option<TQues>, GetQuestionError> {
			if self.endpoint.has_request(){
				return Err(GetQuestionError::AlreadyHasQuestion);
			}
			let question_id = 
				self.queue
				.id_situations_data
				.question_ids
				.first()
				.cloned();

			let data = if let Some(question_id) = question_id{
				let req = unsafe {
					let loc = self.queue.loc_info.id_to_location.get(&question_id).unwrap().clone();
					let slot = self.queue.queue_data.get_unchecked_mut(loc);
					let req = ManuallyDrop::<TQues>::into_inner(slot.assume_init_read().question);
					req
				};

				self.queue.id_situations_data.set_situation(
					question_id,
					Some(RequestSituation::Question),
					Some(RequestSituation::Processing)
				)
				.unwrap();

				self.endpoint.current_req_id = Some(question_id);

				Some(req)
			} else {
				None
			};
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
			.set_situation(req_id, Some(RequestSituation::Processing), Some(RequestSituation::Answer))
			.unwrap();

			Ok(())
		}
	}
}
