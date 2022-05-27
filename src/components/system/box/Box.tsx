import classnames from 'classnames'
import { AllHTMLAttributes, createElement, ElementType } from 'react'
import { sprinkles, Sprinkles } from '../../../styles'

export interface BoxProps
  extends Omit<
  AllHTMLAttributes<HTMLElement>,
  'content' | 'height' | 'translate' | 'color' | 'width' | 'cursor'
  >,
  Sprinkles {
  component?: ElementType
}

export const Box = ({
  component = 'div',
  className,
  padding,
  paddingX,
  paddingY,
  paddingTop,
  paddingBottom,
  paddingLeft,
  paddingRight,
  margin,
  marginX,
  marginY,
  marginTop,
  marginBottom,
  marginLeft,
  marginRight,
  display,
  alignItems,
  justifyContent,
  flexDirection,
  flexWrap,
  flexGrow,
  flexShrink,
  borderRadius,
  position,
  top,
  bottom,
  left,
  right,
  background,
  color,
  width,
  zIndex,
  opacity,
  pointerEvents,
  cursor,
  textAlign,
  maxWidth,
  ...restProps
}: BoxProps) => {
  const atomClasses = classnames(
    sprinkles({
      padding,
      paddingX,
      paddingY,
      paddingTop,
      paddingBottom,
      paddingLeft,
      paddingRight,
      margin,
      marginX,
      marginY,
      marginTop,
      marginBottom,
      marginLeft,
      marginRight,
      display,
      alignItems,
      justifyContent,
      flexDirection,
      flexWrap,
      flexGrow,
      flexShrink,
      borderRadius,
      position,
      top,
      bottom,
      left,
      right,
      background,
      color,
      width,
      zIndex,
      opacity,
      pointerEvents,
      cursor,
      textAlign,
      maxWidth,
    }),
    className,
  )

  return createElement(component, { className: atomClasses, ...restProps })
}
