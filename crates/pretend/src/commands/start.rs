use anyhow::Result;

use crate::output;
use pretend_core::PretendService;

pub fn run(
  service: &PretendService,
  application: &str,
  details: Option<&str>,
  state: Option<&str>,
) -> Result<()> {
  let presence = service.start(
    application,
    details.map(str::to_string),
    state.map(str::to_string),
  )?;
  output::print_start(&presence);
  Ok(())
}
