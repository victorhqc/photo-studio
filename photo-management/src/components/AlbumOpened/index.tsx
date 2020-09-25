import React, { FC, HTMLAttributes } from 'react';
import './styles.css';
import { Album } from '../../store/albums';

const AlbumOpened: FC<Props> = ({ data, ...restOfProps }) => (
  <div className="album-opeened" {...restOfProps}>
    {data.name}
  </div>
);

type Props = OwnProps & HTMLAttributes<HTMLDivElement>;

type OwnProps = {
  data: Album;
};

export default AlbumOpened;
