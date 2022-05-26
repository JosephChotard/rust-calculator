import { createTextStyle } from "@capsizecss/vanilla-extract"
import { style } from "@vanilla-extract/css"
import { queries, responsiveStyle, theme } from "../../styles"

export const makeTypographyRules = (textDefinition: typeof theme.text.standard) => {
  const { fontSize: mobileFontSize, lineHeight: mobileLineHeight } =
    textDefinition.mobile

  const { fontSize: tabletFontSize, lineHeight: tabletLineHeight } =
    textDefinition.tablet

  const { fontSize: desktopFontSize, lineHeight: desktopLineHeight } =
    textDefinition.tablet

  return {
    untrimmed: style(
      responsiveStyle({
        mobile: {
          fontSize: mobileFontSize,
          lineHeight: mobileLineHeight,
        },
        tablet: {
          fontSize: tabletFontSize,
          lineHeight: tabletLineHeight,
        },
        desktop: {
          fontSize: desktopFontSize,
          lineHeight: desktopLineHeight,
        },
      }),
    ),
    trimmed: createTextStyle(textDefinition.mobile, {
      '@media': {
        [queries.tablet]: textDefinition.tablet,
        [queries.desktop]: textDefinition.desktop,
      },
    }),
  }
}