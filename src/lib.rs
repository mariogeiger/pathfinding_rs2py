use ::pathfinding::prelude::{kuhn_munkres, kuhn_munkres_min, Matrix};
use numpy::ndarray::{Array1, ArrayView2};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray2};
use ordered_float::NotNan;
use pyo3::{pymodule, types::PyModule, PyResult, Python};

#[pymodule]
fn pathfinding(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "kuhn_munkres")]
    fn kuhn_munkres_py<'py>(
        py: Python<'py>,
        weights: PyReadonlyArray2<'_, f64>,
    ) -> PyResult<(f64, &'py PyArray1<i32>)> {
        let weights: ArrayView2<f64> = weights.as_array();
        let weights = Matrix::from_rows(weights.rows()).unwrap();
        let weights = weights.map(|x| NotNan::new(*x).unwrap());

        let (cost, assignments) = kuhn_munkres(&weights);

        let cost: f64 = cost.into_inner();
        let assignments: Vec<i32> = assignments.iter().map(|x| *x as i32).collect();
        Ok((cost, Array1::from(assignments).into_pyarray(py)))
    }

    #[pyfn(m)]
    #[pyo3(name = "kuhn_munkres_min")]
    fn kuhn_munkres_min_py<'py>(
        py: Python<'py>,
        weights: PyReadonlyArray2<'_, f64>,
    ) -> PyResult<(f64, &'py PyArray1<i32>)> {
        let weights: ArrayView2<f64> = weights.as_array();
        let weights = Matrix::from_rows(weights.rows()).unwrap();
        let weights = weights.map(|x| NotNan::new(*x).unwrap());

        let (cost, assignments) = kuhn_munkres_min(&weights);

        let cost: f64 = cost.into_inner();
        let assignments: Vec<i32> = assignments.iter().map(|x| *x as i32).collect();
        Ok((cost, Array1::from(assignments).into_pyarray(py)))
    }

    Ok(())
}
