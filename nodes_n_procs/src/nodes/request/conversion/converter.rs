use core::{marker::PhantomData, ops::{Deref, DerefMut}};

use nnp_base::runner::Process;

use crate::request::request::{Request, RequestNMut};

pub enum RequestConversionResult{
	NoConversion,
	ConvertedQuestion,
	ConvertedAnswer
}

pub trait RequestConversion<TQues0, TAnsw0, TQues1, TAnsw1>{
	fn convert_question(&mut self, req: TQues0) -> TQues1;
	fn convert_answer(&mut self, ans: TAnsw1) -> TAnsw0;
	fn convert_req(
		&mut self,
		mut req_0: impl DerefMut<Target = Request<TQues0, TAnsw0>>,
	 	mut req_1: impl DerefMut<Target = Request<TQues1, TAnsw1>>
	) -> RequestConversionResult {
		if req_0.is_question() && req_1.is_standing_by(){
			unsafe{ self.convert_req_question_unchecked(req_0, req_1) };
			RequestConversionResult::ConvertedQuestion
		} else if  req_0.is_waiting() && req_1.is_answer(){
			unsafe{self.convert_req_answer_unchecked(req_1, req_0)};
			RequestConversionResult::ConvertedAnswer
		} else{
			RequestConversionResult::NoConversion
		}
	}
	
	unsafe fn convert_req_question_unchecked(
		&mut self,
		mut req_0: impl DerefMut<Target = Request<TQues0, TAnsw0>>,
	 	mut req_1: impl DerefMut<Target = Request<TQues1, TAnsw1>>
	){ unsafe{
		req_1.put_question_unchecked(self.convert_question(req_0.take_question_unchecked()));
	}}

	// argumentos sao colocados invertidos para evitar confusoes. Eles seguem a ideia de o primeiro vai para o segundo.
	unsafe fn convert_req_answer_unchecked(
		&mut self,
	 	mut req_1: impl DerefMut<Target = Request<TQues1, TAnsw1>>, 
		mut req_0: impl DerefMut<Target = Request<TQues0, TAnsw0>>,
	){ unsafe{
		req_0.put_answer_unchecked(self.convert_answer(req_1.take_answer_unchecked()));
	}}
	
}

impl<TQues0, TAns0, TQues1, TAns1, TFnQues, TFnAnsw>
RequestConversion<TQues0, TAns0, TQues1, TAns1> for (TFnQues, TFnAnsw)
where 
	TFnQues: FnMut(TQues0) -> TQues1,
	TFnAnsw: FnMut(TAns1) -> TAns0,
{
	fn convert_question(&mut self, req: TQues0) -> TQues1 {
		(self.0)(req)
	}

	fn convert_answer(&mut self, ans: TAns1) -> TAns0 {
		(self.1)(ans)
	}
}

// <TQues0, TAnsw0, TQues1, TAnsw1>
pub struct RequestConverter<TQues0, TAnsw0, TQues1, TAnsw1, TConversion>
where 
	TConversion: RequestConversion<TQues0, TAnsw0, TQues1, TAnsw1>
{
	conversion: TConversion,
	__phantom:   PhantomData<(TQues0, TAnsw0, TQues1, TAnsw1)>,
}

impl<TQues0, TAnsw0, TQues1, TAnsw1, TFnQues, TFnAnsw>
RequestConverter<TQues0, TAnsw0, TQues1, TAnsw1, (TFnQues, TFnAnsw)>
where 
	TFnQues: FnMut(TQues0) -> TQues1,
	TFnAnsw: FnMut(TAnsw1) -> TAnsw0,
{
	pub fn new_from_fns(fn_ques: TFnQues, fn_answ: TFnAnsw) -> Self{
		Self::new((fn_ques, fn_answ))
	}
}

impl<TQues0, TAnsw0, TQues1, TAnsw1, TConversion>
RequestConverter<TQues0, TAnsw0, TQues1, TAnsw1, TConversion>
where 
	TConversion: RequestConversion<TQues0, TAnsw0, TQues1, TAnsw1>
{
	pub fn new(conversion: TConversion) -> Self {
		Self { conversion, __phantom: PhantomData }
	}

}



impl<'a, TQues0, TAns0, TQues1, TAns1, TConversion> 
Process<'a> for RequestConverter<TQues0, TAns0, TQues1, TAns1, TConversion>
where 
	TConversion: RequestConversion<TQues0, TAns0, TQues1, TAns1>,
	TQues0: 'a, TAns0: 'a, TQues1: 'a, TAns1: 'a
{
	type TArgs = (
		RequestNMut<'a, TQues0, TAns0>,
		RequestNMut<'a, TQues1, TAns1>
	);

	fn resume(&mut self, (mut req_0, mut req_1): Self::TArgs) {
		self.conversion.convert_req(req_0, req_1);
	}
}



