diesel::table! {
    Contact (UserName) {
        UserName -> Nullable<Text>,
        Alias -> Nullable<Text>,
        EncryptUserName -> Nullable<Text>,
        DelFlag -> Nullable<Integer>,
        Type -> Nullable<Integer>,
        VerifyFlag -> Nullable<Integer>,
        Reserved1 -> Nullable<Integer>,
        Reserved2 -> Nullable<Integer>,
        Reserved3 -> Nullable<Text>,
        Reserved4 -> Nullable<Text>,
        Remark -> Nullable<Text>,
        NickName -> Nullable<Text>,
        LabelIDList -> Nullable<Text>,
        DomainList -> Nullable<Text>,
        ChatRoomType -> Nullable<Integer>,
        PYInitial -> Nullable<Text>,
        QuanPin -> Nullable<Text>,
        RemarkPYInitial -> Nullable<Text>,
        RemarkQuanPin -> Nullable<Text>,
        BigHeadImgUrl -> Nullable<Text>,
        SmallHeadImgUrl -> Nullable<Text>,
        HeadImgMd5 -> Nullable<Text>,
        ChatRoomNotify -> Nullable<Integer>,
        Reserved5 -> Nullable<Integer>,
        Reserved6 -> Nullable<Text>,
        Reserved7 -> Nullable<Text>,
        ExtraBuf -> Nullable<Binary>,
        Reserved8 -> Nullable<Integer>,
        Reserved9 -> Nullable<Integer>,
        Reserved10 -> Nullable<Text>,
        Reserved11 -> Nullable<Text>,
    }
}

diesel::table! {
    Session (strUsrName) {
        strUsrName -> Nullable<Text>,
        nOrder -> Nullable<Integer>,
        nUnReadCount -> Nullable<Integer>,
        parentRef -> Nullable<Text>,
        Reserved0 -> Nullable<Integer>,
        Reserved1 -> Nullable<Text>,
        strNickName -> Nullable<Text>,
        nStatus -> Nullable<Integer>,
        nIsSend -> Nullable<Integer>,
        strContent -> Nullable<Text>,
        nMsgType -> Nullable<Integer>,
        nMsgLocalID -> Nullable<Integer>,
        nMsgStatus -> Nullable<Integer>,
        nTime -> Nullable<Integer>,
        editContent -> Nullable<Text>,
        othersAtMe -> Nullable<Integer>,
        Reserved2 -> Nullable<Integer>,
        Reserved3 -> Nullable<Text>,
        Reserved4 -> Nullable<Integer>,
        Reserved5 -> Nullable<Text>,
        bytesXml -> Nullable<Binary>,
    }
}

diesel::table! {
    ChatRoom (ChatRoomName) {
        ChatRoomName -> Nullable<Text>,
        UserNameList -> Nullable<Text>,
        DisplayNameList -> Nullable<Text>,
        ChatRoomFlag -> Nullable<Integer>,
        Owner -> Nullable<Integer>,
        IsShowName -> Nullable<Integer>,
        SelfDisplayName -> Nullable<Text>,
        Reserved1 -> Nullable<Integer>,
        Reserved2 -> Nullable<Text>,
        Reserved3 -> Nullable<Integer>,
        Reserved4 -> Nullable<Text>,
        Reserved5 -> Nullable<Integer>,
        Reserved6 -> Nullable<Text>,
        RoomData -> Nullable<Binary>,
        Reserved7 -> Nullable<Integer>,
        Reserved8 -> Nullable<Text>,
    }
}
