import React, { useEffect, FC } from 'react';
import { connect } from 'react-redux';
import { Link } from 'react-router-dom';
import { ApplicationState } from '../../store';
import { selectAlbums, fetchAllAlbums } from '../../store/albums';

import './styles.css';

const Albums: FC<Props> = ({ fetchAllAlbums, albums }) => {
  useEffect(() => {
    fetchAllAlbums();
  }, [fetchAllAlbums]);

  return (
    <div className="albums">
      <h1 className="albums__title">Albums</h1>
      <div className="albums__list">
        {albums.map(([album, photos]) => (
          <div className="album__wrapper" data-testid={`album-${album.id}`} key={album.id}>
            <Link to={`/album/${album.id}`}>
              <ul className="album__photos">
                {photos.map((photo) => (
                  <li key={photo.id} className="album__photo__wrapper">
                    <div
                      className="album__photo"
                      style={{ backgroundImage: `url(${photo.src})` }}
                    />
                  </li>
                ))}
              </ul>
            </Link>
            <p className="album__name">{album.name}</p>
            <p className="album__description">{album.description}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

const mapStateToProps = (state: ApplicationState) => ({
  albums: selectAlbums(state),
});

const mapDispatchToProps = {
  fetchAllAlbums: fetchAllAlbums.request,
};

type Props = ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(mapStateToProps, mapDispatchToProps)(Albums);
