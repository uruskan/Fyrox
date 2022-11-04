use crate::define_command_stack;
use fyrox::animation::definition::ResourceTrack;
use fyrox::{
    asset::ResourceDataRef,
    resource::animation::{AnimationResourceError, AnimationResourceState},
};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct AnimationEditorContext<'a> {
    pub resource: ResourceDataRef<'a, AnimationResourceState, AnimationResourceError>,
}

define_command_stack!(
    AnimationCommandTrait,
    AnimationCommandStack,
    AnimationEditorContext
);

#[derive(Debug)]
pub struct AnimationCommand(pub Box<dyn AnimationCommandTrait>);

impl Deref for AnimationCommand {
    type Target = dyn AnimationCommandTrait;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for AnimationCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl AnimationCommand {
    pub fn new<C: AnimationCommandTrait>(cmd: C) -> Self {
        Self(Box::new(cmd))
    }
}

#[derive(Debug)]
pub struct CommandGroup {
    commands: Vec<AnimationCommand>,
}

impl From<Vec<AnimationCommand>> for CommandGroup {
    fn from(commands: Vec<AnimationCommand>) -> Self {
        Self { commands }
    }
}

impl CommandGroup {
    #[allow(dead_code)]
    pub fn push(&mut self, command: AnimationCommand) {
        self.commands.push(command)
    }
}

impl AnimationCommandTrait for CommandGroup {
    fn name(&mut self, context: &AnimationEditorContext) -> String {
        let mut name = String::from("Command group: ");
        for cmd in self.commands.iter_mut() {
            name.push_str(&cmd.name(context));
            name.push_str(", ");
        }
        name
    }

    fn execute(&mut self, context: &mut AnimationEditorContext) {
        for cmd in self.commands.iter_mut() {
            cmd.execute(context);
        }
    }

    fn revert(&mut self, context: &mut AnimationEditorContext) {
        // revert must be done in reverse order.
        for cmd in self.commands.iter_mut().rev() {
            cmd.revert(context);
        }
    }

    fn finalize(&mut self, context: &mut AnimationEditorContext) {
        for mut cmd in self.commands.drain(..) {
            cmd.finalize(context);
        }
    }
}

#[derive(Debug)]
pub struct AddTrackCommand {
    track: Option<ResourceTrack>,
}

impl AddTrackCommand {
    pub fn new(track: ResourceTrack) -> Self {
        Self { track: Some(track) }
    }
}

impl AnimationCommandTrait for AddTrackCommand {
    fn name(&mut self, _: &AnimationEditorContext) -> String {
        "Add Track".to_string()
    }

    fn execute(&mut self, context: &mut AnimationEditorContext) {
        context
            .resource
            .animation_definition
            .tracks_container()
            .push(self.track.take().unwrap());
    }

    fn revert(&mut self, context: &mut AnimationEditorContext) {
        self.track = context
            .resource
            .animation_definition
            .tracks_container()
            .pop();
    }
}
