use core::ops::Deref;

use nnp_base::runner::Process;

use crate::request::{request::{Request, RequestNMut}, request_queue::{request_queue_server::{GetQuestionError, RequestQueueServerEndpoint}, RequestQueueNMut}};

pub struct Queue2SimpleRequestConverter<TQuest, TAns, const QUEUE_SIZE: usize> {
	queue_server_endpoint: RequestQueueServerEndpoint<TQuest, TAns>,
}

impl<'a, TQues, TAns, const QUEUE_SIZE: usize>
Process<'a> for Queue2SimpleRequestConverter<TQues, TAns, QUEUE_SIZE>
where 
	TQues: 'a, TAns: 'a
{
	type TArgs = (
		RequestQueueNMut<'a, TQues, TAns, QUEUE_SIZE>,
		RequestNMut			<'a, TQues, TAns>
	);

	fn resume(&mut self, (queue, mut simple_req): Self::TArgs) {
		if self.queue_server_endpoint.has_request() {
			if simple_req.is_answer(){
				let mut queue_conn  = self.queue_server_endpoint.make_server_connection(queue);
				let answer = simple_req.take_answer().unwrap();
				queue_conn.put_answer(answer).map_err(|_| {()}).unwrap();
			} 
		} else {
			if simple_req.is_standing_by(){
				let mut queue_conn  = self.queue_server_endpoint.make_server_connection(queue);
				if let Some(question) = queue_conn.get_question().unwrap(){
					simple_req.put_question(question).map_err(|_|{}).unwrap();
				} 
			}
		}
	}
}