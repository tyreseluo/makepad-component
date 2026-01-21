use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::theme::colors::*;

    // ============================================================
    // MpAvatar - Avatar/profile picture component
    // ============================================================

    AVATAR_SIZE_XS = 24.0
    AVATAR_SIZE_SM = 32.0
    AVATAR_SIZE_MD = 40.0
    AVATAR_SIZE_LG = 56.0
    AVATAR_SIZE_XL = 80.0

    // Base avatar with initials (circle shape)
    MpAvatarBase = <View> {
        width: (AVATAR_SIZE_MD)
        height: (AVATAR_SIZE_MD)
        align: { x: 0.5, y: 0.5 }

        show_bg: true
        draw_bg: {
            instance radius: 20.0
            instance bg_color: (MUTED)

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let c = self.rect_size * 0.5;
                let r = min(c.x, c.y);

                sdf.circle(c.x, c.y, r);
                sdf.fill(self.bg_color);

                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                color: (MUTED_FOREGROUND)
            }
            text: ""
        }
    }

    // Extra small avatar
    pub MpAvatarXSmall = <MpAvatarBase> {
        width: (AVATAR_SIZE_XS)
        height: (AVATAR_SIZE_XS)

        label = <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 10.0 }
            }
        }
    }

    // Small avatar
    pub MpAvatarSmall = <MpAvatarBase> {
        width: (AVATAR_SIZE_SM)
        height: (AVATAR_SIZE_SM)

        label = <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
            }
        }
    }

    // Default avatar (medium size)
    pub MpAvatar = <MpAvatarBase> {
        width: (AVATAR_SIZE_MD)
        height: (AVATAR_SIZE_MD)

        label = <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
            }
        }
    }

    // Large avatar
    pub MpAvatarLarge = <MpAvatarBase> {
        width: (AVATAR_SIZE_LG)
        height: (AVATAR_SIZE_LG)

        label = <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 20.0 }
            }
        }
    }

    // Extra large avatar
    pub MpAvatarXLarge = <MpAvatarBase> {
        width: (AVATAR_SIZE_XL)
        height: (AVATAR_SIZE_XL)

        label = <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 28.0 }
            }
        }
    }

    // ============================================================
    // Square avatar variants
    // ============================================================

    MpAvatarSquareBase = <View> {
        width: (AVATAR_SIZE_MD)
        height: (AVATAR_SIZE_MD)
        align: { x: 0.5, y: 0.5 }

        show_bg: true
        draw_bg: {
            instance bg_color: (MUTED)
            instance radius: 6.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.radius
                );
                sdf.fill(self.bg_color);

                return sdf.result;
            }
        }

        label = <Label> {
            width: Fit
            height: Fit
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 14.0 }
                color: (MUTED_FOREGROUND)
            }
            text: ""
        }
    }

    pub MpAvatarSquare = <MpAvatarSquareBase> {}

    pub MpAvatarSquareSmall = <MpAvatarSquareBase> {
        width: (AVATAR_SIZE_SM)
        height: (AVATAR_SIZE_SM)

        draw_bg: {
            radius: 4.0
        }

        label = <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 12.0 }
            }
        }
    }

    pub MpAvatarSquareLarge = <MpAvatarSquareBase> {
        width: (AVATAR_SIZE_LG)
        height: (AVATAR_SIZE_LG)

        draw_bg: {
            radius: 8.0
        }

        label = <Label> {
            draw_text: {
                text_style: <THEME_FONT_BOLD> { font_size: 20.0 }
            }
        }
    }

    // ============================================================
    // Color variants
    // ============================================================

    pub MpAvatarPrimary = <MpAvatarBase> {
        draw_bg: {
            bg_color: (PRIMARY)
        }

        label = <Label> {
            draw_text: {
                color: (PRIMARY_FOREGROUND)
            }
        }
    }

    pub MpAvatarDanger = <MpAvatarBase> {
        draw_bg: {
            bg_color: (DANGER)
        }

        label = <Label> {
            draw_text: {
                color: (DANGER_FOREGROUND)
            }
        }
    }

    pub MpAvatarSuccess = <MpAvatarBase> {
        draw_bg: {
            bg_color: (SUCCESS)
        }

        label = <Label> {
            draw_text: {
                color: (SUCCESS_FOREGROUND)
            }
        }
    }

    pub MpAvatarWarning = <MpAvatarBase> {
        draw_bg: {
            bg_color: (WARNING)
        }

        label = <Label> {
            draw_text: {
                color: (WARNING_FOREGROUND)
            }
        }
    }

    // ============================================================
    // Avatar Group
    // ============================================================

    pub MpAvatarGroup = <View> {
        width: Fit
        height: Fit
        flow: Right
        spacing: -12  // Negative spacing for overlap
        align: { y: 0.5 }
    }
}
