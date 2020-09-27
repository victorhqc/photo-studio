import React, { FC, useEffect } from 'react';
import { connect } from 'react-redux';
import { ApplicationState } from '../../store';
import { selectAlbums, fetchAllAlbums } from '../../store/albums';
import Album from '../../components/Album';
import AlbumOpened from '../../components/AlbumOpened';
import './styles.css';

const HomeView: FC<Props> = ({ fetchAllAlbums, albums }) => {
  useEffect(() => {
    fetchAllAlbums();
  }, [fetchAllAlbums]);

  if (albums.length === 0) {
    return null;
  }

  if (albums.length === 1) {
    return (
      <div className="home__default-album">
        <AlbumOpened data={albums[0][0]} />
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

const mapDispatchToProps = {
  fetchAllAlbums: fetchAllAlbums.request,
};

const mapStateToProps = (state: ApplicationState) => ({
  albums: selectAlbums(state),
});

type Props = ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(mapStateToProps, mapDispatchToProps)(HomeView);
