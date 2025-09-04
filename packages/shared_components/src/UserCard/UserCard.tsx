import React from 'react';
import './UserCard.css';

export interface UserCardProps {
  name: string;
  avatar?: string;
  description?: string;
}

export const UserCard: React.FC<UserCardProps> = ({ name, avatar, description }) => {
  return (
    <div className="user-card">
      {avatar && <img src={avatar} alt={name} className="user-avatar" />}
      <h3 className="user-name">{name}</h3>
      {description && <p className="user-description">{description}</p>}
    </div>
  );
};
