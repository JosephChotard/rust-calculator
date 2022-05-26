import { createTextStyle } from '@capsizecss/vanilla-extract'
import { style, styleVariants } from '@vanilla-extract/css'
import { mapToProperty, queries, responsiveStyle, theme } from '../../styles'

const makeTypographyRules = (textDefinition: typeof theme.text.standard) => {
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
      // @ts-ignore TS 4.5
      '@media': {
        [queries.tablet]: textDefinition.tablet,
        [queries.desktop]: textDefinition.desktop,
      },
    }),
  }
}

export const font = styleVariants(theme.fonts, mapToProperty('fontFamily'))
export const weight = styleVariants(theme.weight, mapToProperty('fontWeight'))

export const text = {
  standard: makeTypographyRules(theme.text.standard),
  small: makeTypographyRules(theme.text.small),
  xsmall: makeTypographyRules(theme.text.xsmall),
  code: makeTypographyRules(theme.text.code),
}

export const heading = {
  '1': makeTypographyRules(theme.heading.h1),
  '2': makeTypographyRules(theme.heading.h2),
  '3': makeTypographyRules(theme.heading.h3),
  '4': makeTypographyRules(theme.heading.h4),
}
