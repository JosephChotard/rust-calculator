import { style, styleVariants } from '@vanilla-extract/css'
import { sprinkles, theme } from '../../../styles'

const baseButtonStyle = style([
  sprinkles({
    text: {
      mobile: 'small',
      tablet: 'medium',
      desktop: 'large',
    }
  }),
  {
    border: 'none',
    borderRadius: '8px',
    padding: '8px 16px',
    fontWeight: 'bold',
    cursor: 'pointer',
    outline: 'none',
  }
])

export const buttonStyle = styleVariants({
  primary: [
    baseButtonStyle,
    {
      backgroundColor: theme.contract.colors.primary,
      ':hover': {
        backgroundColor: '#eaeaea',
      }
    }
  ],
  secondary: [
    baseButtonStyle,
    {
      backgroundColor: theme.contract.colors.secondary,
      ':hover': {
        backgroundColor: '#de99ff',
      }
    }
  ]
})