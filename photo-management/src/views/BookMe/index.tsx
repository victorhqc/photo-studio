import React, { FC, FormEvent, ChangeEvent, useEffect, useCallback, useState } from 'react';
import { connect } from 'react-redux';
import { selectBookMeEmail, fetchBookMeInfo, updateBookMeInfo } from '../../store/bookMe';
import { ApplicationState } from '../../store';
import './styles.css';

const BookMe: FC<Props> = ({ fetchBookMeInfo, updateBookMeInfo, email }) => {
  const [bookMeInfo, setBookMeInfo] = useState<FormData>({ email });

  const hasChanged = email !== bookMeInfo.email;

  useEffect(() => {
    fetchBookMeInfo();
  }, [fetchBookMeInfo]);

  useEffect(() => {
    setBookMeInfo((state) => ({
      ...state,
      email,
    }));
  }, [email]);

  const handleSubmit = useCallback(
    (e: FormEvent) => {
      e.preventDefault();

      if (!bookMeInfo.email) {
        return;
      }

      updateBookMeInfo({ email: bookMeInfo.email });
    },
    [updateBookMeInfo, bookMeInfo]
  );

  const handleChange = useCallback((key: FormKey) => {
    return (e: ChangeEvent<HTMLInputElement>) => {
      setBookMeInfo((state) => ({
        ...state,
        [key]: e.target.value,
      }));
    };
  }, []);

  return (
    <div className="book_me">
      <h1 className="book_me__title">Book Me</h1>
      <form onSubmit={handleSubmit} className="form">
        <div className="form__input_box">
          <label className="form__label" htmlFor="email">
            Email
          </label>
          <input
            className="form__input"
            id="email"
            name="email"
            type="email"
            onChange={handleChange('email')}
            value={bookMeInfo.email || ''}
          />
        </div>
        <div className="form__input_box">
          <button className="form__btn" disabled={!hasChanged}>
            Save
          </button>
        </div>
      </form>
    </div>
  );
};

const mapStateToProps = (state: ApplicationState) => ({
  email: selectBookMeEmail(state),
});

const mapDispatchToProps = {
  fetchBookMeInfo: fetchBookMeInfo.request,
  updateBookMeInfo: updateBookMeInfo.request,
};

type Props = ReturnType<typeof mapStateToProps> & typeof mapDispatchToProps;

export default connect(mapStateToProps, mapDispatchToProps)(BookMe);

type FormData = { email?: string };
type FormKey = keyof FormData;
