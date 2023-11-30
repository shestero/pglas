use lasrs::Las;
use pgrx::iter::TableIterator;
use pgrx::*;
use std::fs::File;

// pgx specific macros
pg_module_magic!();

fn vec_of_result_to_result_of_vec<T, E>(v: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    v.into_iter().collect()
}

fn op_of_result_to_result_of_op<T, E>(o: Option<Result<T, E>>) -> Result<Option<T>, E> {
    o.map_or(Ok(None), |v| v.map(Some))
}

fn filter(v: f64, na: Option<f64>) -> Option<f64> {
    if Some(v) == na {
        None
    } else {
        Some(v)
    }
}

#[pg_extern]
fn las_na(
    file: &'static str,
) -> core::result::Result<Option<f64>, Box<dyn std::error::Error + 'static>> {
    let _ = File::open(file)?;
    let las = Las::new(file);
    let na = las
        .well_info()
        .get("NULL")
        .map(|prop| prop.value.parse::<f64>());
    op_of_result_to_result_of_op(na)
        .map_err(|e| format!("Cannot parse NULL (N/A) value {}!", e).into())
}

#[pg_extern]
fn las_curves(
    file: &'static str,
) -> core::result::Result<
    TableIterator<'static, (name!(IDX, i64), name!(CURVE, String))>,
    Box<dyn std::error::Error + 'static>,
> {
    let _ = File::open(file)?;
    let las = Las::new(file);
    let v: Vec<(i64, String)> = (0i64..).zip(las.headers().into_iter()).collect();
    Ok(TableIterator::new(v))
}

#[pg_extern]
fn las_curve(
    file: &'static str,
    curve: String,
) -> core::result::Result<
    TableIterator<'static, (name!(DEPT, Option<f64>), name!(VAL, Option<f64>))>,
    Box<dyn std::error::Error + 'static>,
> {
    let _ = File::open(file)?;
    let las = Las::new(file);
    let na: Option<f64> = las
        .well_info()
        .get("NULL")
        .and_then(|prop| prop.value.parse::<f64>().ok());
    let header = las.headers();
    if header.first() != Some(&"DEPT".to_string()) {
        return Err("DEPT isn't the first field/column!".into());
    }
    let index = header
        .iter()
        .position(|col| curve == *col)
        .ok_or(format!("there is no field '{}' in '{}'", curve, file))?;
    let v0: Vec<Result<(Option<f64>, Option<f64>), Box<dyn std::error::Error + 'static>>> = las
        .data()
        .into_iter()
        .map(|d| {
            let c1 = d
                .iter()
                .nth(0)
                .ok_or(format!("No data in file {}!", file))?
                .clone();
            let c2 = d
                .iter()
                .nth(index)
                .ok_or(format!(
                    "Cannot read column #{} ({}) in file {}!",
                    index, curve, file
                ))?
                .clone();
            Ok((filter(c1, na), filter(c2, na)))
        })
        .collect();
    let v = vec_of_result_to_result_of_vec(v0)?;

    // let v: Vec<(f64, f64)> = dept.iter().copied().zip(val.iter().copied()).collect();
    Ok(TableIterator::new(v))
}
