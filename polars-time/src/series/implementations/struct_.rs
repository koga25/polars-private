use super::*;

impl SeriesOpsTime for Wrap<StructChunked> {
    fn ops_time_dtype(&self) -> &DataType {
        self.0.dtype()
    }
}
