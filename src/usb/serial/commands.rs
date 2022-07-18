use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DataTransmitRequestArgs {
    ProfileID(String),
    GlobalSettings,
    BankSettings(u8),
}

impl Display for DataTransmitRequestArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            DataTransmitRequestArgs::ProfileID(s) => format!("profileId,{s}"),
            DataTransmitRequestArgs::GlobalSettings => "globalSettings".to_string(),
            DataTransmitRequestArgs::BankSettings(x) => format!("bankSettings,{x}"),
        };
        write!(f, "{output}")
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DataRequestArgs {
    GlobalSettings,
    BankSettings(i8),
}

impl Display for DataRequestArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            DataRequestArgs::GlobalSettings => "globalSettings".to_string(),
            DataRequestArgs::BankSettings(x) => format!("bankSettings,{x}"),
        };
        write!(f, "{output}")
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ControlArgs {
    BankUp,
    BankDown,
    GoToBank(i8),
    ToggleFootswitch(i8),
    DeviceRestart,
    EnterBootloader,
    FactoryReset,
}

impl Display for ControlArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ControlArgs::BankUp => "bankUp".to_string(),
            ControlArgs::BankDown => "bankDown".to_string(),
            ControlArgs::GoToBank(x) => format!("goToBank,{x}"),
            ControlArgs::ToggleFootswitch(x) => format!("toggleFootswitch,{x}"),
            ControlArgs::DeviceRestart => "deviceRestart".to_string(),
            ControlArgs::EnterBootloader => "enterBootloader".to_string(),
            ControlArgs::FactoryReset => "factoryReset".to_string(),
        };
        write!(f, "{output}")
    }
}

/// Command is the basis of our commands
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Command {
    //
    Check,
    Control(ControlArgs),
    DataRequest(DataRequestArgs),
    DataTransmitRequest(DataTransmitRequestArgs),
    Reset,
}

impl Command {
    pub fn format(&self) -> Vec<String> {
        match self {
            Command::Check => vec![format!("{self}")],
            Command::Control(args) => match args {
                _ => vec![format!("{self}"), format!("{args}")],
            },
            Command::DataRequest(args) => match args {
                _ => vec![format!("{self}"), format!("{args}")],
            },
            Command::DataTransmitRequest(args) => match args {
                _ => vec![format!("{self}"), format!("{args}")],
            },
            Command::Reset => vec![format!("{self}")],
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Command::Check => "CHCK",
            Command::Control(_) => "CTRL",
            Command::DataRequest(_) => "DREQ",
            Command::DataTransmitRequest(_) => "DTXR",
            Command::Reset => "RSET",
        };
        write!(f, "{output}")
    }
}
