use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
use crate::queue::logic::api::queue_search_criteria::QueueSearchCriteria;
use suborbital::db;
use suborbital::db::query;
use anyhow::Result;

pub struct QueueService;

impl Service<Vec<u8>, QueueSearchCriteria, i64> for QueueService {
    fn get_by_id(id: i64) -> Result<Option<Vec<u8>>> {

        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", &id.to_string()));

        let queue_query_result = db::select("SelectQueue", query_args);
        return match queue_query_result {
            Ok(query_res) => {
                if query_res.len() > 2 {
                    Ok(Some(query_res[1..query_res.len() - 1].to_vec()))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(anyhow::Error::msg(e.message))
        }
    }

    fn search(search_criteria: QueueSearchCriteria) -> Result<SearchResult<Vec<u8>>> {
        Ok(SearchResult {
            content: vec![],
            pageable: Pageable {
                page_size: 0,
                page_number: 0,
                sort: None
            },
            total_elements: 0
        })
    }

    fn delete(id: i64) -> Result<i64> {
        let mut query_args1: Vec<query::QueryArg> = Vec::new();
        let mut query_args2: Vec<query::QueryArg> = Vec::new();
        query_args1.push(query::QueryArg::new("id", &id.to_string()));
        query_args2.push(query::QueryArg::new("id", &id.to_string()));

        let queue_query_1_result = db::delete("DeleteACQueue", query_args1);
        let queue_query_2_result = db::delete("DeleteQueue", query_args2);
        return match queue_query_1_result {
            Ok(query_res) => {
                return match queue_query_2_result {
                    Ok(query_res) => {
                        Ok(id)
                    }
                    Err(e) => Err(anyhow::Error::msg(e.message))
                }
            }
            Err(e) => Err(anyhow::Error::msg(e.message))
        }
    }

    fn create(eto: Vec<u8>) -> Result<Vec<u8>> {
        Ok(eto)
    }
}