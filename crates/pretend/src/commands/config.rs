use anyhow::Result;

use crate::output;
use pretend_core::PretendService;

pub fn run(service: &PretendService) -> Result<()> {
  let summary = service.config_summary();
  output::print_config(&summary);
  Ok(())
}
