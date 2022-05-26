import { styleVariants } from '@vanilla-extract/css'
import { mapToProperty, theme } from '../../styles'
import { makeTypographyRules } from './utils.css'


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
