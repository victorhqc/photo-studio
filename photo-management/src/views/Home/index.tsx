import React, { FC, useEffect } from 'react';
import { connect } from 'react-redux';
import { ApplicationState } from '../../store';
import { selectAlbums, fetchAlbums } from '../../store/albums';
import Album from '../../components/Album';
import AlbumOpened from '../../components/AlbumOpened';

const HomeView: FC<Props> = ({ fetchAlbums, albums }) => {
  useEffect(() => {
    fetchAlbums();
  }, [fetchAlbums]);

  if (albums.length === 0) {
    return null;
  }

  if (albums.length === 1) {
    return <AlbumOpened data={albums[0][0]} />;
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
  fetchAlbums: fetchAlbums.request,
};

const mapStateToProps = (state: ApplicationState) => ({
  albums: selectAlbums(state),
});

type Props = ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(mapStateToProps, mapDispatchToProps)(HomeView);
