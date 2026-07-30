use anyhow::Result;

use crate::output;
use pretend_core::PretendService;

pub fn run(service: &PretendService) -> Result<()> {
  let apps = service.list();
  output::print_apps(&apps);
  Ok(())
}
