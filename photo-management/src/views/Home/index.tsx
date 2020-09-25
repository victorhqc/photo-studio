import React, { FC, useEffect } from 'react';
import { connect } from 'react-redux';
import { ApplicationState } from '../../store';
import { selectAlbums, fetchAlbums } from '../../store/albums';

const HomeView: FC<Props> = ({ fetchAlbums, albums }) => {
  useEffect(() => {
    fetchAlbums();
  }, [fetchAlbums]);

  console.log({ albums });
  const defaultAlbum = albums[0];
  console.log(defaultAlbum);

  if (!defaultAlbum) {
    return null;
  }

  return (
    <>
      <h1>Home</h1>
    </>
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
