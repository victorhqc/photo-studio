import React, { FC, ReactNode } from 'react';
import { connect } from 'react-redux';
import { ApplicationState } from '../../store';
import { selectMaybeAuthenticatedUser } from '../../store/auth';

const Authentication: FC<Props> = ({ user, notAuthed, children }) => {
  if (!user) {
    return <>{notAuthed}</>;
  }

  return <>{children}</>;
};

const mapStateToProps = (state: ApplicationState) => ({
  user: selectMaybeAuthenticatedUser(state),
});

type Props = ReturnType<typeof mapStateToProps> & OwnProps;

type OwnProps = {
  notAuthed: ReactNode;
  children?: ReactNode;
};

export default connect(mapStateToProps)(Authentication);
