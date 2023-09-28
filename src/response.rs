use crate::stream::Stream;

pub struct Response<'r> {
   stream: Box<&'r mut Stream<'r>>
}

impl Response<'static> {
    pub fn new(stream: &mut Stream) { 

    }
}