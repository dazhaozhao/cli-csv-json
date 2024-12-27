use rcli::csv::CsvTransformer;

fn main() -> anyhow::Result<()> {
  //rcli csv -i ./input.csv -o ./output.json -split ","
  CsvTransformer::transform()
}
