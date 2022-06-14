type RootStackParamListType = {
    Welcome: undefined;
    SignUp: undefined;
    LogIn: undefined;
    Home: undefined;
    Footer: undefined;
    Button: undefined;
    ToSignUpOrLoginTextArea: undefined;
    AuthErrorText: undefined;
    ForgotPassword: undefined;
    Chats: undefined;
    AddGroup: {groupName: string, groupImage: string, backFriendList: FriendListPropsType[]};
    AddGroupSetting: { backGroupName: string, backGroupImage: string, friendList: FriendListPropsType[]};
    AddFriend: undefined;
    Profile: undefined;
    EditNickName: undefined;
    Chat: { groupChatRoomId: string, directChatRoomId: string, profileImage: string, name: string, groupMemberUserId: string[], addGroupMemberName: string};
    AddGroupMember: undefined;
    AlreadyFriendModal: { user: string, groupChatRoomId: string, groupImage: string, groupName: string, directChatRoomId: string };
    NotFriendModal:  { user: string, groupChatRoomId: string, groupImage: string, groupName: string, directChatRoomId: string|null };
  };
  