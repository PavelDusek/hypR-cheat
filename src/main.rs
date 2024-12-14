use iced::widget::{
    scrollable,
    Column, Row, Scrollable, Text,
};
use iced::{Color, Length,};

#[derive(Debug)]
enum Message {
}

#[derive(Default, Debug, Clone)]
struct Binding {
    mod1: String,
    mod2: String,
    key: String,
    command: String,
}
impl Binding {
    fn create_binding( mod1: &str, mod2: &str, key: &str, command: &str ) -> Binding {
        Binding {
            mod1: mod1.to_string(),
            mod2: mod2.to_string(),
            key: key.to_string(),
            command: command.to_string(),
        }
    }
}

#[derive(Default, Debug)]
struct BindingList {
    list: Vec<Binding>,
}
impl BindingList {
    fn create() -> BindingList {
        let binding_list = vec![
            Binding::create_binding( "Super", "", "Q", "exec, $terminal"),
            Binding::create_binding( "Super", "", "M", "exit, "),
            Binding::create_binding( "Super", "", "E", "exec, $fileManager"),
            Binding::create_binding( "Super", "", "V", "togglefloating, "),
            Binding::create_binding( "Super", "", "P", "pseudo, # dwindle"),
            Binding::create_binding( "Super", "", "left", "movefocus, l"),
            Binding::create_binding( "Super", "", "right", "movefocus, r"),
            Binding::create_binding( "Super", "", "up", "movefocus, u"),
            Binding::create_binding( "Super", "", "down", "movefocus, d"),
            Binding::create_binding( "Super", "", "Return", "exec, $terminal"),
            Binding::create_binding( "Super", "", "B", "exec, zen-browser #firefox"),
            Binding::create_binding( "Super", "Shift"," Q", "killactive,"),
            Binding::create_binding( "Super", "", "D", "exec, $menu"),
            Binding::create_binding( "Super", "Shift", "B", "exec, $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "C", "exec, rofi -show calc -moci calc -no-show-match -no-sort"),
            Binding::create_binding( "Super", "", "Z", "exec, $VOLUME --add -10"),
            Binding::create_binding( "Super", "Shift", "Z", "exec, $VOLUME --add +10"),
            Binding::create_binding( "Super", "", "O", "exec, dolphin"),
            Binding::create_binding( "Super", "Shift", "T", "togglesplit, #dwindle"),
            Binding::create_binding( "Super", "", "T", "exec, tilix"),
            Binding::create_binding( "Super", "", "H", "movefocus, l"),
            Binding::create_binding( "Super", "", "J", "movefocus, d"),
            Binding::create_binding( "Super", "", "K", "movefocus, u"),
            Binding::create_binding( "Super", "", "L", "movefocus, r"),
            Binding::create_binding( "Super", "", "F", "fullscreen, 0"),
            Binding::create_binding( "Super", "", "1", "workspace, 1"),
            Binding::create_binding( "Super", "", "1", "exec, $w1 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "2", "workspace, 2"),
            Binding::create_binding( "Super", "", "2", "exec, $w2 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "3", "workspace, 3"),
            Binding::create_binding( "Super", "", "3", "exec, $w3 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "4", "workspace, 4"),
            Binding::create_binding( "Super", "", "4", "exec, $w4 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "5", "workspace, 5"),
            Binding::create_binding( "Super", "", "5", "exec, $w5 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "6", "workspace, 6"),
            Binding::create_binding( "Super", "", "6", "exec, $w6 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "7", "workspace, 7"),
            Binding::create_binding( "Super", "", "7", "exec, $w7 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "8", "workspace, 8"),
            Binding::create_binding( "Super", "", "8", "exec, $w8 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "9", "workspace, 9"),
            Binding::create_binding( "Super", "", "9", "exec, $w9 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "", "0", "workspace, 10"),
            Binding::create_binding( "Super", "", "0", "exec, $w0 & $RANDOM_WALL"),
            Binding::create_binding( "Super", "Shift", "1", "movetoworkspace, 1"),
            Binding::create_binding( "Super", "Shift", "2", "movetoworkspace, 2"),
            Binding::create_binding( "Super", "Shift", "3", "movetoworkspace, 3"),
            Binding::create_binding( "Super", "Shift", "4", "movetoworkspace, 4"),
            Binding::create_binding( "Super", "Shift", "5", "movetoworkspace, 5"),
            Binding::create_binding( "Super", "Shift", "6", "movetoworkspace, 6"),
            Binding::create_binding( "Super", "Shift", "7", "movetoworkspace, 7"),
            Binding::create_binding( "Super", "Shift", "8", "movetoworkspace, 8"),
            Binding::create_binding( "Super", "Shift", "9", "movetoworkspace, 9"),
            Binding::create_binding( "Super", "Shift", "0", "movetoworkspace, 10"),
            Binding::create_binding( "Super", "", "S", "togglespecialworkspace, magic"),
            Binding::create_binding( "Super", "Shift", "S", "movetoworkspace, special:magic"),
            Binding::create_binding( "Super", "", "mouse_down", "workspace, e+1"),
            Binding::create_binding( "Super", "", "mouse_up", "workspace, e-1"),
            Binding::create_binding( "Super", "", "mouse:272", "movewindow"),
            Binding::create_binding( "Super", "", "mouse:273", "resizewindow"),
            Binding::create_binding( "Super", "Shift", "G", "exec, grim"),
            Binding::create_binding( "Super", "", "G", "exec, grim -g \"$(slurp -d)\""),
            Binding::create_binding( "Super", "CTRL", "G", "exec, hyprctl -j activewindow | jq -r '\"\\(.at[0]),\\(.at[1]) \\(.size[0])x\\(.size[1])\"' | grim"),
        ];
        BindingList{ list: binding_list, }
    }
    fn update(&mut self, _message: Message) {
    }
    fn view(&self) -> Scrollable<Message> {
        let binding_list = BindingList::create();
        let mut v = Vec::new();
        for element in binding_list.list {
            v.push(
                Row::new()
                    .spacing(10)
                    .push(Text::new(element.mod1).color(Color::from_rgb(1.0, 0.0, 0.0)))
                    .push(Text::new(element.mod2).color(Color::from_rgb(0.0, 1.0, 0.0)))
                    .push(Text::new(element.key).color(Color::from_rgb(0.0, 0.0, 1.0)))
                    .push(Text::new(element.command))
                    .into()
            );
        }
        scrollable(Column::from_vec(v)).width(Length::Fill).into()
    }
}

fn main() -> iced::Result {
    iced::run( "hypR-cheat", BindingList::update, BindingList::view)
}
