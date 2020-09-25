import React, { FC, HTMLAttributes } from 'react';
import './styles.css';
import { Album } from '../../store/albums';

const AlbumComponent: FC<Props> = ({ data, ...restOfProps }) => (
  <div className="album-opened" {...restOfProps}>
    {data.name}
  </div>
);

type Props = OwnProps & HTMLAttributes<HTMLDivElement>;

type OwnProps = {
  data: Album;
};

export default AlbumComponent;
