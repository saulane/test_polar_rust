use polars::lazy::dsl::*;
use polars::lazy::prelude::*;
use polars::prelude::Expr::Nth;
use polars::prelude::PolarsResult;
use std::ops::Mul;
use std::time;

use connectorx::prelude::*;
use std::convert::TryFrom;

// fn main() -> PolarsResult<()> {
//     let q = LazyCsvReader::new("test.csv")
//         .finish()?
//         .select(&[all() - all().mean()])
//         .select(&[col("*").exclude(&["0"]).mul(col("0")).sum()
//             / (col("0")
//                 .count()
//                 .map(|v| Ok(Some(v - 1)), GetOutput::default())
//                 * Nth(0).std(1)
//                 * col("*").exclude(&["0"]).std(1))]);
//     let now = time::Instant::now();

//     println!("{}", q.with_streaming(true).collect()?);

//     println!("Time elapsed: {:?}", now.elapsed());

//     Ok(())
// }

fn main() {
    let mut source_conn =
        SourceConn::try_from("mysql://default:default@localhost:9004/default?cxprotocol=text")
            .expect("parse conn str failed");
    let queries = &[CXQuery::from("SELECT * from my_first_table")];
    let destination = get_arrow2(&source_conn, None, queries).expect("run failed");

    let data = destination.polars();
    println!("{:?}", data);
}
