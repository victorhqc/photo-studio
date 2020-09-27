import React, { FC, HTMLAttributes } from 'react';
import './styles.css';

const PhotoGrid: FC<Props> = ({ children, className, ...props }) => (
  <div className={`photo-grid ${className || ''}`} {...props}>
    {children}
  </div>
);

type Props = HTMLAttributes<HTMLDivElement>;

export default PhotoGrid;

export const PhotoColumn: FC<Props> = ({ children, className, ...props }) => (
  <div className={`photo-column ${className || ''}`} {...props}>
    {children}
  </div>
);
