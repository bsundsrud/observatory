use crate::config::FileConfig;
use crate::logging;
use crate::model::World;
use crate::vizceral::types::VizceralNode;
use actix_web::{dev::Resource, App, HttpRequest, HttpResponse, Path, Responder};
use serde_derive::Deserialize;
use slog::Logger;
use std::fs::File;
use std::sync::{Arc, RwLock};

pub struct ObservatoryState {
    logger: Logger,
    graphs: GraphState,
}

impl ObservatoryState {
    pub fn new(logger: Logger) -> ObservatoryState {
        let world_logger = logger.new(o!("context" => "graph-state"));
        let mut world = World::new(world_logger);
        let f = File::open("sample/simple.yml").unwrap();
        let c: FileConfig = serde_yaml::from_reader(&f).unwrap();
        world.extend_from_config(&c);
        ObservatoryState {
            logger: logger,
            graphs: GraphState::new(world),
        }
    }
}

#[derive(Clone)]
pub struct GraphState {
    graphs: Arc<RwLock<World>>,
}

impl GraphState {
    fn new(world: World) -> GraphState {
        GraphState {
            graphs: Arc::new(RwLock::new(world)),
        }
    }

    pub fn to_vizceral(&self, name: &str) -> Option<VizceralNode> {
        let graph = self.graphs.clone();
        let graph = graph.read().unwrap();
        graph.graph(name).map(VizceralNode::from)
    }
}

#[derive(Debug, Deserialize)]
pub struct StateParams {
    name: String,
}

pub fn state_for_name(
    (req, params): (HttpRequest<ObservatoryState>, Path<StateParams>),
) -> impl Responder {
    if let Some(v) = req.state().graphs.to_vizceral(&params.name) {
        HttpResponse::Ok().json(v)
    } else {
        HttpResponse::NotFound().json(format!("Couldn't find {}", params.name))
    }
}

pub fn get_app(logger: Logger) -> App<ObservatoryState> {
    let web_logger = logger.new(o!("context" => "request"));

    App::with_state(ObservatoryState::new(logger.clone()))
        .middleware(logging::RequestLogger::new(web_logger))
        .resource("/api/state/{name}", |r: &mut Resource<_>| {
            r.get().with(state_for_name)
        })
}
