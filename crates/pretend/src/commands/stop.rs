use anyhow::Result;

use crate::output;
use pretend_core::PretendService;

pub fn run(service: &PretendService) -> Result<()> {
  service.stop()?;
  output::print_stop();
  Ok(())
}
