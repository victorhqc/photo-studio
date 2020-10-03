import React, { FC, FormEvent, useCallback, useEffect, useRef, useState } from 'react';
import { connect } from 'react-redux';
import { PlusIcon, SyncIcon } from '@primer/octicons-react';
import { ApplicationState } from '../../store';
import {
  addPhoto,
  buildApplication,
  selectUploadStatus,
  selectNeedsRebuild,
} from '../../store/albums';
import { getColorFrom } from '../../utils/chameleon';
import PhotoGrid from '../PhotoGrid';
import './styles.css';

const AddPhoto: FC<Props> = ({ addPhoto, buildApplication, needsRebuild, status }) => {
  const inputRef = useRef<HTMLInputElement | null>(null);
  const [imagePreview, setImagePreview] = useState<{
    base64: ArrayBuffer | string;
    color: string;
  } | null>(null);

  useEffect(() => {
    if (status === 'done' && inputRef.current) {
      inputRef.current.value = '';
      setImagePreview(null);
    }
  }, [status]);

  useEffect(() => {
    const handleUnload = (e: BeforeUnloadEvent) => {
      if (!needsRebuild) return;

      e.preventDefault();
      e.returnValue = '';
    };

    window.addEventListener('beforeunload', handleUnload);

    return () => {
      window.removeEventListener('beforeunload', handleUnload);
    };
  }, [needsRebuild]);

  const handleClick = useCallback((e: FormEvent) => {
    e.preventDefault();

    if (!inputRef.current) return;

    inputRef.current.click();
  }, []);

  const handleFileChange = useCallback(() => {
    if (!inputRef.current || !inputRef.current.files) return;

    const file = inputRef.current.files[0];

    const fr = new FileReader();
    fr.onload = async () => {
      const color = await getColorFrom(fr.result as string);
      setImagePreview({ base64: fr.result as string, color });
    };

    fr.readAsDataURL(file);
  }, []);

  const handleConfirm = useCallback(
    (e: FormEvent) => {
      e.preventDefault();

      if (!inputRef.current || !inputRef.current.files || !imagePreview) return;

      const img = inputRef.current.files[0];

      addPhoto({
        img,
        color: imagePreview.color,
      });
    },
    [addPhoto, imagePreview]
  );

  const handleCancel = useCallback((e: FormEvent) => {
    e.preventDefault();

    if (!inputRef.current) return;

    inputRef.current.value = '';
    setImagePreview(null);
  }, []);

  const handleRebuild = useCallback(
    (e: FormEvent) => {
      e.preventDefault();

      buildApplication();
    },
    [buildApplication]
  );

  return (
    <form className="add-photo">
      <PhotoGrid>
        {status === 'loading' && (
          <div className="loading">
            <SyncIcon className="rotate-infinite" size="large" />
          </div>
        )}
        {imagePreview ? (
          <>
            <div
              className="add-photo__preview"
              style={{ backgroundImage: `url(${imagePreview.base64})` }}
            >
              <div className="add-photo__confirm-info">
                <div className="add-photo__btns-wrapper">
                  <button
                    className="add-photo__confirm-btn add-photo__confirm-btn--accept"
                    onClick={handleConfirm}
                    disabled={status === 'loading'}
                  >
                    Upload
                  </button>
                  <button
                    className="add-photo__confirm-btn add-photo__confirm-btn--cancel"
                    onClick={handleCancel}
                  >
                    Cancel
                  </button>
                </div>
              </div>
            </div>
          </>
        ) : (
          <button className="add-photo_button" onClick={handleClick}>
            <h1>Add photo</h1>
            <PlusIcon size="medium" />
          </button>
        )}
        <input
          ref={inputRef}
          type="file"
          accept="image/*"
          className="add-photo__input"
          onChange={handleFileChange}
        />
        <div className="add-photo__rebuild-wrapper">
          <h2 className="add-photo__rebuild-title">
            {needsRebuild ? 'Rebuild Website' : 'No rebuild is needed'}
          </h2>
          <p className="add-photo__rebuild-text">
            {needsRebuild
              ? 'To reflect the changes in photos, a rebuild in the website is needed.'
              : 'No changes done in photos, so no rebuild is needed in the website.'}
          </p>
          {needsRebuild && (
            <button className="add-photo__rebuild-button" onClick={handleRebuild}>
              Rebuild Website
            </button>
          )}
        </div>
      </PhotoGrid>
    </form>
  );
};

const mapDispatchToProps = {
  addPhoto: addPhoto.request,
  buildApplication,
};

const mapStateToProps = (state: ApplicationState) => ({
  status: selectUploadStatus(state),
  needsRebuild: selectNeedsRebuild(state),
});

type Props = ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(mapStateToProps, mapDispatchToProps)(AddPhoto);
