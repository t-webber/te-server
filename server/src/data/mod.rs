use crate::AddRoutes;
use axum::Router;
use std::marker::PhantomData;

pub mod mdl;
pub mod users;

pub struct DataBaseRouter();

impl AddRoutes<DataBaseRouter> for Router {
    fn add_routes(self, _: PhantomData<DataBaseRouter>) -> Self {
        self.add_routes(PhantomData::<mdl::MdlRouter>)
            .add_routes(PhantomData::<users::UserRouter>)
    }
}
