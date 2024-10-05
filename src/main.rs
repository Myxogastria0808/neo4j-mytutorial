use neo4rs::*;

#[tokio::main]
async fn main() {
    let uri = "localhost:7687";
    let user = "neo4j";
    let pass = "123456789";
    let graph = Graph::new(&*uri, user, pass).await.unwrap();

    // for id in 1..=7 {
    //     graph
    //         .run(query("CREATE (item:Item {id: $id})").param("id", id))
    //         .await
    //         .unwrap();
    // }
    // graph
    //     .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
    //     .param("parent_id", 1).param("child_id", 2))
    //     .await
    //     .unwrap();
    // graph
    //     .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
    //     .param("parent_id", 1).param("child_id", 6))
    //     .await
    //     .unwrap();
    // graph
    //     .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
    //     .param("parent_id", 6).param("child_id", 7))
    //     .await
    //     .unwrap();
    // graph
    //     .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
    //     .param("parent_id", 1).param("child_id", 5))
    //     .await
    //     .unwrap();
    // graph
    //     .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
    //     .param("parent_id", 5).param("child_id", 3))
    //     .await
    //     .unwrap();
    // graph
    //     .run(query("MATCH (child:Item {id: $child_id}) MATCH (parent:Item {id: $parent_id}) CREATE (child)-[relation:ItemTree]->(parent)")
    //     .param("parent_id", 5).param("child_id", 4))
    //     .await
    //     .unwrap();

    //ノードのパターン
    {
        {
            let mut result = graph
                .execute(query("MATCH (item) RETURN item"))
                .await
                .unwrap();
            loop {
                let item = result.next().await.unwrap();
                let row = match item {
                    Some(row) => row,
                    None => break,
                };
                let node = row.get::<Node>("item").unwrap();
                let id = node.get::<i64>("id").unwrap();
                println!("{:#?}", id);
            }
        }
        //整理した状態
        {
            let mut result = graph
                .execute(query("MATCH (item) RETURN item"))
                .await
                .unwrap();
            loop {
                let id = match result
                    .next()
                    .await
                    .expect("[ERROR]: Unexpected error was occured.")
                {
                    Some(row) => row
                        .get::<Node>("item")
                        .expect("[ERROR]: Cannot get value of 'item' (key name).")
                        .get::<i64>("id")
                        .expect("[ERROR]: Cannot get value of 'id' (key name)."),
                    None => break,
                };
                println!("id: {}", id);
            }
        }
    }

    //関係のパターン
    {
        {
            let mut result = graph
                .execute(query("MATCH path=({id:7})-[*]->() RETURN path"))
                .await
                .unwrap();
            let mut row = match result.next().await.unwrap() {
                Some(row) => row,
                None => panic!("[ERROR]: Row not found."),
            };
            loop {
                row = match result.next().await.unwrap() {
                    Some(row) => row,
                    None => break,
                };
            }
            let nodes = row.get::<Path>("path").unwrap().nodes();
            for node in nodes {
                let id = node
                    .get::<i64>("id")
                    .expect("[ERROR]: Cannot get value of 'id' (key name).");
                println!("path: {}", id);
            }
        }
        {
            let mut result = graph
                .execute(query("MATCH path=({id:7})-[*]->() RETURN path"))
                .await
                .unwrap();
            let mut row = match result.next().await.unwrap() {
                Some(row) => row,
                None => panic!("[ERROR]: Row not found."),
            };
            loop {
                row = match result.next().await.unwrap() {
                    Some(row) => row,
                    None => break,
                }
            }
            let id: Vec<i64> = row
                .get::<Path>("path")
                .unwrap()
                .nodes()
                .into_iter()
                .map(|node| {
                    node.get::<i64>("id")
                        .expect("[ERROR]: Cannot get value of 'item' (key name).")
                })
                .collect();
            println!("{:?}", id);
        }
    }
}
