pub struct SampleImpl {
    myName:String
}

impl SampleImpl{
    pub fn new(name:String) -> SampleImpl{
        SampleImpl{myName: name}
    }

    pub fn hello_world(&self){
        println!("My name is: {:?}",self.myName);
    }
}
