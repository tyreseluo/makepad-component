use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpNotification - Toast/notification component
    // ============================================================

    // Base notification
    MpNotificationBase = <View> {
        width: 320
        height: Fit
        padding: 16
        flow: Right
        spacing: 12
        align: { y: 0.0 }

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_radius: 8.0
            instance border_color: (BORDER)
            instance shadow_color: #0000001A
            instance shadow_offset_y: 4.0
            instance shadow_blur: 12.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow
                sdf.box(
                    0.0,
                    self.shadow_offset_y,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                );
                sdf.blur = self.shadow_blur;
                sdf.fill(self.shadow_color);
                sdf.blur = 0.0;

                // Main card
                sdf.box(
                    0.5,
                    0.5,
                    self.rect_size.x - 1.0,
                    self.rect_size.y - 1.0,
                    self.border_radius
                );
                sdf.fill_keep(self.bg_color);
                sdf.stroke(self.border_color, 1.0);

                return sdf.result;
            }
        }
    }

    // ============================================================
    // Default Notification
    // ============================================================

    pub MpNotification = <MpNotificationBase> {
        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (MUTED_FOREGROUND)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let r = min(c.x, c.y) - 1.0;

                    // Info icon (circle with i)
                    sdf.circle(c.x, c.y, r);
                    sdf.stroke(self.icon_color, 1.5);

                    // Letter i
                    sdf.circle(c.x, c.y - 3.0, 1.5);
                    sdf.fill(self.icon_color);
                    sdf.rect(c.x - 1.0, c.y, 2.0, 5.0);
                    sdf.fill(self.icon_color);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (FOREGROUND)
                }
                text: "Notification"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: #94a3b8
                instance hover: 0.0

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let size = 5.0;

                    let final_color = mix(self.icon_color, #64748b, self.hover);

                    // X mark
                    sdf.move_to(c.x - size, c.y - size);
                    sdf.line_to(c.x + size, c.y + size);
                    sdf.stroke(final_color, 1.5);

                    sdf.move_to(c.x + size, c.y - size);
                    sdf.line_to(c.x - size, c.y + size);
                    sdf.stroke(final_color, 1.5);

                    return sdf.result;
                }
            }

            animator: {
                hover = {
                    default: off
                    off = {
                        from: { all: Forward { duration: 0.15 } }
                        apply: { draw_bg: { hover: 0.0 } }
                    }
                    on = {
                        from: { all: Forward { duration: 0.1 } }
                        apply: { draw_bg: { hover: 1.0 } }
                    }
                }
            }
        }
    }

    // ============================================================
    // Success Notification
    // ============================================================

    pub MpNotificationSuccess = <MpNotificationBase> {
        draw_bg: {
            border_color: #bbf7d0
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (SUCCESS)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Checkmark
                    sdf.move_to(c.x - 5.0, c.y);
                    sdf.line_to(c.x - 1.0, c.y + 4.0);
                    sdf.line_to(c.x + 6.0, c.y - 4.0);
                    sdf.stroke(self.icon_color, 2.0);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (SUCCESS)
                }
                text: "Success"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
        }
    }

    // ============================================================
    // Error Notification
    // ============================================================

    pub MpNotificationError = <MpNotificationBase> {
        draw_bg: {
            border_color: #fecaca
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (DANGER)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let r = min(c.x, c.y) - 1.0;

                    // Circle
                    sdf.circle(c.x, c.y, r);
                    sdf.stroke(self.icon_color, 1.5);

                    // X mark
                    let size = 4.0;
                    sdf.move_to(c.x - size, c.y - size);
                    sdf.line_to(c.x + size, c.y + size);
                    sdf.stroke(self.icon_color, 1.5);

                    sdf.move_to(c.x + size, c.y - size);
                    sdf.line_to(c.x - size, c.y + size);
                    sdf.stroke(self.icon_color, 1.5);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (DANGER)
                }
                text: "Error"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
        }
    }

    // ============================================================
    // Warning Notification
    // ============================================================

    pub MpNotificationWarning = <MpNotificationBase> {
        draw_bg: {
            border_color: #fde68a
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (WARNING)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;

                    // Triangle
                    sdf.move_to(c.x, 2.0);
                    sdf.line_to(self.rect_size.x - 2.0, self.rect_size.y - 2.0);
                    sdf.line_to(2.0, self.rect_size.y - 2.0);
                    sdf.close_path();
                    sdf.stroke(self.icon_color, 1.5);

                    // Exclamation mark
                    sdf.rect(c.x - 1.0, 7.0, 2.0, 5.0);
                    sdf.fill(self.icon_color);
                    sdf.circle(c.x, 15.0, 1.5);
                    sdf.fill(self.icon_color);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: #b45309
                }
                text: "Warning"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
        }
    }

    // ============================================================
    // Info Notification
    // ============================================================

    pub MpNotificationInfo = <MpNotificationBase> {
        draw_bg: {
            border_color: #a5f3fc
        }

        icon = <View> {
            width: 20
            height: 20
            align: { x: 0.5, y: 0.5 }

            show_bg: true
            draw_bg: {
                instance icon_color: (INFO)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let c = self.rect_size * 0.5;
                    let r = min(c.x, c.y) - 1.0;

                    // Circle
                    sdf.circle(c.x, c.y, r);
                    sdf.stroke(self.icon_color, 1.5);

                    // Letter i
                    sdf.circle(c.x, c.y - 3.0, 1.5);
                    sdf.fill(self.icon_color);
                    sdf.rect(c.x - 1.0, c.y, 2.0, 5.0);
                    sdf.fill(self.icon_color);

                    return sdf.result;
                }
            }
        }

        content = <View> {
            width: Fill
            height: Fit
            flow: Down
            spacing: 4

            title = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (INFO)
                }
                text: "Info"
            }

            message = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: ""
            }
        }

        close = <View> {
            width: 20
            height: 20
            cursor: Hand
        }
    }

    // ============================================================
    // Notification Container (for positioning)
    // ============================================================

    pub MpNotificationContainer = <View> {
        width: Fill
        height: Fill
        flow: Overlay

        // Top-right corner positioning
        align: { x: 1.0, y: 0.0 }
        padding: 16

        notifications = <View> {
            width: Fit
            height: Fit
            flow: Down
            spacing: 8
        }
    }
}
