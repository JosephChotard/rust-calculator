import classnames from 'classnames'
import { ElementType, FC, ReactNode } from 'react'
import { sprinkles, Sprinkles } from '../../styles'
import { colours, textColours } from '../../styles/theme'
import { Box } from '../system/box/Box'
import * as styles from './typography.css'



interface TextStyleProps {
  size?: keyof typeof styles.text
  color?: keyof typeof textColours | { lightMode: keyof typeof colours, darkMode: keyof typeof colours }
  weight?: keyof typeof styles.weight
  align?: Sprinkles['textAlign']
  baseline?: boolean
  type?: Exclude<keyof typeof styles.font, 'brand' | 'heading'>
  invertedColour?: boolean
  className?: string
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
  invertedColour = false,
}: TextStyleProps) =>
  classnames(
    styles.font[type],
    baseline ? styles.text[size].trimmed : styles.text[size].untrimmed,
    sprinkles({
      color: typeof color === 'string' ? (invertedColour ? {
        lightMode: textColours[color].darkMode,
        darkMode: textColours[color].lightMode,
      } : textColours[color]) : color,
      textAlign: align
    }),
    styles.weight[weight],
  )

const Text: FC<TextProps> = ({
  component = 'span',
  invertedColour,
  size,
  color,
  weight,
  align,
  baseline = true,
  type,
  className,
  children,
}) => {
  return (
    <Box
      component={component}
      className={classnames(useTextStyles({ size, color, weight, type, align, baseline, invertedColour }), className)}
    >
      {children}
    </Box>
  )
}

export default Text
