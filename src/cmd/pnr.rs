// use anyhow::{Context, Result};
// use std::process::Command;
// use structopt::StructOpt;
// use crate::cmd::{Execute, PNR};

// #[derive(Debug, StructOpt)]
// pub enum PnrCommand {
//     #[structopt(about = "Run place and route")]
//     Run {
//         #[structopt(short, long, help = "Input JSON file")]
//         json: String,
//         #[structopt(short, long, help = "Input PCF file")]
//         pcf: String,
//         #[structopt(short, long, help = "Output ASC file")]
//         asc: String,
//         #[structopt(short, long, help = "FPGA architecture (e.g., ice40, ecp5)")]
//         arch: String,
//         #[structopt(long, help = "FPGA package")]
//         package: Option<String>,
//         #[structopt(long, help = "Enable timing-driven placement")]
//         timing_driven: bool,
//     },
//     #[structopt(about = "Generate timing report")]
//     TimingReport {
//         #[structopt(short, long, help = "Input JSON file")]
//         json: String,
//         #[structopt(short, long, help = "Output report file")]
//         report: String,
//         #[structopt(short, long, help = "FPGA architecture (e.g., ice40, ecp5)")]
//         arch: String,
//     },
//     #[structopt(about = "Generate utilization report")]
//     UtilizationReport {
//         #[structopt(short, long, help = "Input JSON file")]
//         json: String,
//         #[structopt(short, long, help = "Output report file")]
//         report: String,
//         #[structopt(short, long, help = "FPGA architecture (e.g., ice40, ecp5)")]
//         arch: String,
//     },
// }

// impl Execute for PNR {
//     async fn execute(&self) -> Result<()> {
//         match &self.command {
//             PnrCommand::Run { json, pcf, asc, arch, package, timing_driven } => {
//                 run_pnr(json, pcf, asc, arch, package, *timing_driven)
//             },
//             PnrCommand::TimingReport { json, report, arch } => {
//                 generate_timing_report(json, report, arch)
//             },
//             PnrCommand::UtilizationReport { json, report, arch } => {
//                 generate_utilization_report(json, report, arch)
//             },
//         }
//     }
// }

// fn run_pnr(json: &str, pcf: &str, asc: &str, arch: &str, package: &Option<String>, timing_driven: bool) -> Result<()> {
//     let mut cmd = Command::new("nextpnr");
//     cmd.arg(format!("--{}", arch))
//        .arg("--json").arg(json)
//        .arg("--pcf").arg(pcf)
//        .arg("--asc").arg(asc);

//     if let Some(pkg) = package {
//         cmd.arg("--package").arg(pkg);
//     }

//     if timing_driven {
//         cmd.arg("--timing-allow-fail");
//     }

//     let status = cmd.status().context("Failed to run nextpnr")?;
//     if !status.success() {
//         anyhow::bail!("nextpnr failed with exit code: {}", status);
//     }

//     Ok(())
// }

// fn generate_timing_report(json: &str, report: &str, arch: &str) -> Result<()> {
//     let status = Command::new("nextpnr")
//         .arg(format!("--{}", arch))
//         .arg("--json").arg(json)
//         .arg("--timing-report").arg(report)
//         .status()
//         .context("Failed to generate timing report")?;

//     if !status.success() {
//         anyhow::bail!("Timing report generation failed with exit code: {}", status);
//     }

//     Ok(())
// }

// fn generate_utilization_report(json: &str, report: &str, arch: &str) -> Result<()> {
//     let status = Command::new("nextpnr")
//         .arg(format!("--{}", arch))
//         .arg("--json").arg(json)
//         .arg("--write").arg(report)
//         .status()
//         .context("Failed to generate utilization report")?;

//     if !status.success() {
//         anyhow::bail!("Utilization report generation failed with exit code: {}", status);
//     }

//     Ok(())
// }

use anyhow::{Context, Result};
use std::process::Command;
use crate::cmd::{Execute, Pnr};

impl Execute for Pnr {
    async fn execute(&self) -> Result<()> {
        println!("Running NextPNR...");
        run_nextpnr(self)
    }
}

fn run_nextpnr(pnr: &Pnr) -> Result<()> {
    let mut cmd = Command::new("nextpnr");
    
    cmd.arg(format!("--{}", pnr.arch))
       .arg("--json").arg(&pnr.json)
       .arg("--asc").arg(&pnr.asc);

    if let Some(pcf) = &pnr.pcf {
        cmd.arg("--pcf").arg(pcf);
    }

    if let Some(package) = &pnr.package {
        cmd.arg("--package").arg(package);
    }

    if pnr.timing_driven {
        cmd.arg("--timing-allow-fail");
    }

    if let Some(report) = &pnr.timing_report {
        cmd.arg("--timing-report").arg(report);
    }

    if let Some(report) = &pnr.utilization_report {
        cmd.arg("--write").arg(report);
    }

    let status = cmd.status().context("Failed to run nextpnr")?;
    if !status.success() {
        anyhow::bail!("nextpnr failed with exit code: {}", status);
    }

    println!("NextPNR completed successfully.");
    Ok(())
}