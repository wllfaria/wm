use crate::event::EventContext;
use crate::handlers::handler::Handler;
use config::keysyms::Keysym;
use config::AvailableActions;

#[derive(Default)]
pub struct ActionHandler {}

impl Handler for ActionHandler {
    fn on_key_press(&mut self, context: EventContext<xcb::x::KeyPressEvent>) -> anyhow::Result<()> {
        let keysym = context
            .keyboard
            .state
            .key_get_one_sym(context.event.detail().into());

        if let Ok(keysym) = Keysym::try_from(keysym) {
            if let Some(action) = context
                .config
                .actions()
                .iter()
                .find(|action| action.key().eq(&keysym))
            {
                match action.action() {
                    AvailableActions::Close => {
                        // attempt to close the active client, if theres none, we do nothing
                        if let Some(client) =
                            context.screen_manager.borrow_mut().close_focused_client()?
                        {
                            context.layout_manager.close_client(client, &context)?;
                            context
                                .layout_manager
                                .display_screens(&context.screen_manager, context.decorator)?;
                        }
                    }
                    AvailableActions::FocusLeft => todo!(),
                    AvailableActions::FocusDown => todo!(),
                    AvailableActions::FocusUp => todo!(),
                    AvailableActions::FocusRight => todo!(),
                    AvailableActions::MoveLeft => todo!(),
                    AvailableActions::MoveDown => todo!(),
                    AvailableActions::MoveUp => todo!(),
                    AvailableActions::MoveRight => todo!(),
                    AvailableActions::Reload => todo!(),
                    AvailableActions::Workspace1 => todo!(),
                    AvailableActions::Workspace2 => todo!(),
                    AvailableActions::Workspace3 => todo!(),
                    AvailableActions::Workspace4 => todo!(),
                    AvailableActions::Workspace5 => todo!(),
                    AvailableActions::Workspace6 => todo!(),
                    AvailableActions::Workspace7 => todo!(),
                    AvailableActions::Workspace8 => todo!(),
                    AvailableActions::Workspace9 => todo!(),
                    AvailableActions::Workspace0 => todo!(),
                }
            }
        }

        Ok(())
    }
}
