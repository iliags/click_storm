use egui_snarl::ui::{PinInfo, SnarlViewer};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ClickStormNode {
    TestNode,
}

impl ClickStormNode {
    #[allow(unused)]
    const fn name(&self) -> &str {
        match self {
            ClickStormNode::TestNode => "Test Node",
        }
    }
}

pub struct ClickStormViewer;

impl SnarlViewer<ClickStormNode> for ClickStormViewer {
    fn title(&mut self, _node: &ClickStormNode) -> String {
        String::from("Test Node")
    }

    fn inputs(&mut self, _node: &ClickStormNode) -> usize {
        0
    }

    #[allow(refining_impl_trait)]
    fn show_input(
        &mut self,
        _pin: &egui_snarl::InPin,
        _ui: &mut egui::Ui,
        _snarl: &mut egui_snarl::Snarl<ClickStormNode>,
    ) -> PinInfo {
        PinInfo::default()
    }

    fn outputs(&mut self, _node: &ClickStormNode) -> usize {
        0
    }

    #[allow(refining_impl_trait)]
    fn show_output(
        &mut self,
        _pin: &egui_snarl::OutPin,
        _ui: &mut egui::Ui,
        _snarl: &mut egui_snarl::Snarl<ClickStormNode>,
    ) -> PinInfo {
        PinInfo::default()
    }
}
