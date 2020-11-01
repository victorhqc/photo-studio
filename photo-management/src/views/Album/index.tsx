import React, { FC } from 'react';
import { connect } from 'react-redux';
import { ApplicationState } from '../../store';
import { selectAlbums } from '../../store/albums';
import Album from '../../components/Album';
import AlbumOpened from '../../components/AlbumOpened';
import './styles.css';

const HomeView: FC<Props> = ({ albums }) => {
  if (albums.length === 0) {
    return null;
  }

  if (albums.length === 1) {
    return (
      <div className="home__default-album">
        <AlbumOpened albumId={albums[0][0].id} />
      </div>
    );
  }

  return (
    <div data-testid="home">
      {albums.map(([album]) => (
        <Album key={album.id} data={album} />
      ))}
    </div>
  );
};

const mapStateToProps = (state: ApplicationState) => ({
  albums: selectAlbums(state),
});

type Props = ReturnType<typeof mapStateToProps>;

export default connect(mapStateToProps)(HomeView);
