pub enum BrokerActionType{
    RegisterBroker,
    UnRegisterBroker,
}

pub struct BrokerInfo{
    pub broker_id: u64,
    pub broker_ip: String
}


