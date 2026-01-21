use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpPopover - Popover/Dropdown panel component
    // ============================================================

    // Base popover container
    MpPopoverBase = <View> {
        width: Fit
        height: Fit
        padding: 8

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
    // Default Popover
    // ============================================================

    pub MpPopover = <MpPopoverBase> {
        width: 240
        height: Fit
        padding: 12
        flow: Down
        spacing: 8
    }

    // Popover with arrow pointing up
    pub MpPopoverArrowUp = <View> {
        width: Fit
        height: Fit
        flow: Down
        align: { x: 0.5 }

        arrow = <View> {
            width: 16
            height: 8
            margin: { bottom: -1 }

            show_bg: true
            draw_bg: {
                instance arrow_color: (CARD)
                instance arrow_border_color: (BORDER)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let w = self.rect_size.x;
                    let h = self.rect_size.y;

                    // Triangle pointing up
                    sdf.move_to(w * 0.5, 0.0);
                    sdf.line_to(w, h);
                    sdf.line_to(0.0, h);
                    sdf.close_path();
                    sdf.fill_keep(self.arrow_color);
                    sdf.stroke(self.arrow_border_color, 1.0);

                    return sdf.result;
                }
            }
        }

        content = <MpPopoverBase> {
            width: 240
            height: Fit
            padding: 12
            flow: Down
            spacing: 8
        }
    }

    // Popover with arrow pointing down
    pub MpPopoverArrowDown = <View> {
        width: Fit
        height: Fit
        flow: Down
        align: { x: 0.5 }

        content = <MpPopoverBase> {
            width: 240
            height: Fit
            padding: 12
            flow: Down
            spacing: 8
        }

        arrow = <View> {
            width: 16
            height: 8
            margin: { top: -1 }

            show_bg: true
            draw_bg: {
                instance arrow_color: (CARD)
                instance arrow_border_color: (BORDER)

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    let w = self.rect_size.x;
                    let h = self.rect_size.y;

                    // Triangle pointing down
                    sdf.move_to(0.0, 0.0);
                    sdf.line_to(w, 0.0);
                    sdf.line_to(w * 0.5, h);
                    sdf.close_path();
                    sdf.fill_keep(self.arrow_color);
                    sdf.stroke(self.arrow_border_color, 1.0);

                    return sdf.result;
                }
            }
        }
    }

    // ============================================================
    // Popover Menu (for dropdown menus)
    // ============================================================

    pub MpPopoverMenu = <MpPopoverBase> {
        width: 200
        height: Fit
        padding: 4
        flow: Down
    }

    // Menu item
    pub MpPopoverMenuItem = <View> {
        width: Fill
        height: Fit
        padding: { left: 12, right: 12, top: 8, bottom: 8 }
        flow: Right
        align: { y: 0.5 }
        spacing: 8
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: #00000000
            instance bg_color_hover: #f1f5f9
            instance border_radius: 4.0
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let result_color = mix(self.bg_color, self.bg_color_hover, self.hover);
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(result_color);
                return sdf.result;
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: { all: Forward { duration: 0.1 } }
                    apply: { draw_bg: { hover: 0.0 } }
                }
                on = {
                    from: { all: Forward { duration: 0.05 } }
                    apply: { draw_bg: { hover: 1.0 } }
                }
            }
        }

        label = <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
                color: (FOREGROUND)
            }
            text: "Menu Item"
        }
    }

    // Danger menu item
    pub MpPopoverMenuItemDanger = <MpPopoverMenuItem> {
        draw_bg: {
            bg_color_hover: #fef2f2
        }

        label = <Label> {
            draw_text: {
                color: (DANGER)
            }
        }
    }

    // Menu divider
    pub MpPopoverMenuDivider = <View> {
        width: Fill
        height: 1
        margin: { top: 4, bottom: 4 }
        show_bg: true
        draw_bg: {
            color: (BORDER)
        }
    }

    // Menu section header
    pub MpPopoverMenuHeader = <View> {
        width: Fill
        height: Fit
        padding: { left: 12, right: 12, top: 8, bottom: 4 }

        <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 11.0 }
                color: (MUTED_FOREGROUND)
            }
        }
    }

    // ============================================================
    // Popover Content Variants
    // ============================================================

    // Simple text popover (for tooltips with more content)
    pub MpPopoverText = <MpPopoverBase> {
        width: 240
        height: Fit
        padding: 12

        <Label> {
            width: Fill
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                color: (FOREGROUND)
            }
            text: "Popover content"
        }
    }

    // Popover with header
    pub MpPopoverWithHeader = <MpPopoverBase> {
        width: 280
        height: Fit
        flow: Down

        header = <View> {
            width: Fill
            height: Fit
            padding: { left: 12, right: 12, top: 12, bottom: 8 }

            title_label = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                    color: (FOREGROUND)
                }
                text: "Popover Title"
            }
        }

        body = <View> {
            width: Fill
            height: Fit
            padding: { left: 12, right: 12, top: 0, bottom: 12 }

            desc_label = <Label> {
                width: Fill
                height: Fit
                draw_text: {
                    text_style: <THEME_FONT_REGULAR> { font_size: 13.0 }
                    color: (MUTED_FOREGROUND)
                }
                text: "Popover description text."
            }
        }
    }
}
