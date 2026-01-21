use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpCard - Card container component
    // ============================================================

    // Base card style (using RoundedView for border support)
    pub MpCard = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: (CARD)
            border_radius: 8.0
        }
    }

    // Card with shadow
    pub MpCardShadow = <View> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance border_radius: 8.0
            instance shadow_color: #0000001A
            instance shadow_offset_x: 0.0
            instance shadow_offset_y: 2.0
            instance shadow_blur: 8.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                // Shadow (drawn first, behind the card)
                let shadow_x = self.shadow_offset_x;
                let shadow_y = self.shadow_offset_y;
                sdf.box(
                    shadow_x,
                    shadow_y,
                    self.rect_size.x - abs(shadow_x),
                    self.rect_size.y - abs(shadow_y),
                    self.border_radius
                );
                sdf.blur = self.shadow_blur;
                sdf.fill(self.shadow_color);
                sdf.blur = 0.0;

                // Main card
                sdf.box(0.0, 0.0, self.rect_size.x, self.rect_size.y, self.border_radius);
                sdf.fill(self.bg_color);

                return sdf.result;
            }
        }
    }

    // Card with hover effect
    pub MpCardHover = <View> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12
        cursor: Hand

        show_bg: true
        draw_bg: {
            instance bg_color: (CARD)
            instance bg_color_hover: #f8fafc
            instance border_radius: 8.0
            instance border_color: (BORDER)
            instance hover: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let bg = mix(self.bg_color, self.bg_color_hover, self.hover);

                sdf.box(
                    0.5,
                    0.5,
                    self.rect_size.x - 1.0,
                    self.rect_size.y - 1.0,
                    self.border_radius
                );
                sdf.fill_keep(bg);
                sdf.stroke(self.border_color, 1.0);

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

    // Outlined card (no background fill)
    pub MpCardOutline = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #00000000
            border_radius: 8.0
            border_color: (BORDER)
        }
    }

    // Ghost card (transparent, no border)
    pub MpCardGhost = <View> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12
    }

    // ============================================================
    // Card sub-components
    // ============================================================

    // Card header section
    pub MpCardHeader = <View> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 4
    }

    // Card title
    pub MpCardTitle = <Label> {
        width: Fit
        height: Fit
        draw_text: {
            text_style: <THEME_FONT_BOLD> { font_size: 18.0 }
            color: (CARD_FOREGROUND)
        }
    }

    // Card description
    pub MpCardDescription = <Label> {
        width: Fill
        height: Fit
        draw_text: {
            text_style: <THEME_FONT_REGULAR> { font_size: 14.0 }
            color: (MUTED_FOREGROUND)
        }
    }

    // Card content section
    pub MpCardContent = <View> {
        width: Fill
        height: Fit
        flow: Down
        spacing: 8
    }

    // Card footer section
    pub MpCardFooter = <View> {
        width: Fill
        height: Fit
        flow: Right
        spacing: 8
        align: { y: 0.5 }
    }

    // ============================================================
    // Size Variants
    // ============================================================

    // Small card
    pub MpCardSmall = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 12
        spacing: 8

        draw_bg: {
            color: (CARD)
            border_radius: 6.0
        }
    }

    // Large card
    pub MpCardLarge = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 24
        spacing: 16

        draw_bg: {
            color: (CARD)
            border_radius: 12.0
        }
    }

    // ============================================================
    // Color Variants
    // ============================================================

    // Primary card
    pub MpCardPrimary = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: (PRIMARY)
            border_radius: 8.0
        }
    }

    // Danger card
    pub MpCardDanger = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #fef2f2
            border_radius: 8.0
            border_color: #fecaca
        }
    }

    // Success card
    pub MpCardSuccess = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #f0fdf4
            border_radius: 8.0
            border_color: #bbf7d0
        }
    }

    // Warning card
    pub MpCardWarning = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #fffbeb
            border_radius: 8.0
            border_color: #fde68a
        }
    }

    // Info card
    pub MpCardInfo = <RoundedView> {
        width: Fill
        height: Fit
        flow: Down
        padding: 16
        spacing: 12

        draw_bg: {
            color: #ecfeff
            border_radius: 8.0
            border_color: #a5f3fc
        }
    }
}
