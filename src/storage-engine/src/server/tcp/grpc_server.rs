use protocol::storage_engine::storage::{
    storage_engine_service_server::StorageEngineService, CreateSegmentRequest,
    CreateSegmentResponse, CreateShardRequest, CreateShardResponse, DeleteSegmentRequest,
    DeleteSegmentResponse, ReadRequest, ReadResponse, ShardDetailRequest, ShardDetailResponse,
    WriteRequest, WriteResponse,
};
use tonic::{Request, Response, Status};

pub struct StorageService {}

impl StorageService {
    pub fn new() -> Self {
        return StorageService {};
    }
}

#[tonic::async_trait]
impl StorageEngineService for StorageService {
    async fn write(
        &self,
        request: Request<WriteRequest>,
    ) -> Result<Response<WriteResponse>, Status> {
        return Ok(Response::new(WriteResponse::default()));
    }

    async fn read(&self, request: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        return Ok(Response::new(ReadResponse::default()));
    }

    async fn create_shard(
        &self,
        request: Request<CreateShardRequest>,
    ) -> Result<Response<CreateShardResponse>, Status> {
        return Ok(Response::new(CreateShardResponse::default()));
    }

    async fn describe_shard(
        &self,
        request: Request<ShardDetailRequest>,
    ) -> Result<Response<ShardDetailResponse>, Status> {
        return Ok(Response::new(ShardDetailResponse::default()));
    }

    async fn create_segment(
        &self,
        request: Request<CreateSegmentRequest>,
    ) -> Result<Response<CreateSegmentResponse>, Status> {
        return Ok(Response::new(CreateSegmentResponse::default()));
    }

    async fn delete_segment(
        &self,
        request: Request<DeleteSegmentRequest>,
    ) -> Result<Response<DeleteSegmentResponse>, Status> {
        return Ok(Response::new(DeleteSegmentResponse::default()));
    }
}
