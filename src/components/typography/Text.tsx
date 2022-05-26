import classnames from 'classnames'
import { ElementType, FC, ReactNode } from 'react'
import { sprinkles, Sprinkles } from '../../styles'
import { Box } from '../system/box/Box'
import * as styles from './typography.css'

const colorMap = {
  neutral: { lightMode: 'grey700', darkMode: 'grey100' },
  strong: { lightMode: 'grey900', darkMode: 'grey100' },
  code: { lightMode: 'white' },
  link: { lightMode: 'grey700', darkMode: 'grey100' },
  secondary: { lightMode: 'grey500', darkMode: 'grey400' },
  highlight: { lightMode: 'pink500' },
} as const

interface TextStyleProps {
  size?: keyof typeof styles.text
  color?: keyof typeof colorMap
  weight?: keyof typeof styles.weight
  align?: Sprinkles['textAlign']
  baseline?: boolean
  type?: Exclude<keyof typeof styles.font, 'brand' | 'heading'>
}

export interface TextProps extends TextStyleProps {
  component?: ElementType
  children: ReactNode
}

export const useTextStyles = ({
  size = 'standard',
  color = 'neutral',
  weight = 'regular',
  type = 'body',
  align,
  baseline = true,
}: TextStyleProps) =>
  classnames(
    styles.font[type],
    baseline ? styles.text[size].trimmed : styles.text[size].untrimmed,
    sprinkles({ color: colorMap[color], textAlign: align }),
    styles.weight[weight],
  )

const Text: FC<TextProps> = ({
  component = 'span',
  size,
  color,
  weight,
  align,
  baseline = true,
  type,
  children,
}) => {
  return (
    <Box
      component={component}
      // display="block"
      className={useTextStyles({ size, color, weight, type, align, baseline })}
    >
      {children}
    </Box>
  )
}

export default Text
