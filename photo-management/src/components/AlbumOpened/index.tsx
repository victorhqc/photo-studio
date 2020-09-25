import React, { FC, HTMLAttributes } from 'react';
import './styles.css';
import { Album } from '../../store/albums';
import AddPhoto from '../AddPhoto';

const AlbumOpened: FC<Props> = ({ data, ...restOfProps }) => (
  <div className="album-opened" {...restOfProps}>
    <AddPhoto />
  </div>
);

type Props = OwnProps & HTMLAttributes<HTMLDivElement>;

type OwnProps = {
  data: Album;
};

export default AlbumOpened;
