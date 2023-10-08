use crate::method::TypeConverter;
use crate::{WIPICContext, WIPICWord};
use core::mem;

/// Note: Enum names and comments are taken directly from WIPI documentation.
#[repr(i32)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum WIPICErrorCode {
    /// 성공
    SUCCESS = 0,
    /// 기타에러
    ERROR = -1,
    /// 잘못된 식별자
    BADFD = -2,
    /// 잘못된 파일 이름
    BADFILENAME = -3,
    /// 잘못된 파일 위치
    BADSEEKPOS = -4,
    /// 해당 리소스가 이미 존재함
    EXIST = -5,
    /// 잘못된 포맷
    BADFORMAT = -6,
    /// 오퍼레이션 수행중
    INPROGRESS = -7,
    /// 현재 사용중이거나 이미 사용중
    INUSE = -8,
    /// 매개변수가 잘못되었음
    INVALID = -9,
    /// 이미 연결이 설정되어 있음
    ISCONN = -10,
    /// 제한길이 초과
    LONGNAME = -11,
    /// 내용 없음
    NOENT = -12,
    /// 남은 공간이 없음
    NOSPACE = -13,
    /// 연결이 설정되어 있지 않음
    NOTCONN = -14,
    /// 비어있지 않음
    NOTEMPTY = -15,
    /// 해당 서비스를 지원하지 않음
    NOTSUP = -16,
    /// 메모리 부족
    NOMEMORY = -17,
    /// 버퍼가 작음
    SHORTBUF = -18,
    /// WOULDBLOCK 발생
    WOULDBLOCK = -19,
    /// 타임아웃
    TIMEOUT = -20,
    /// 데이터가 너무 큼
    DATABIG = -21,
    /// 잘못된 레코드 식별자
    BADRECID = -22,
    /// 파일의 끝
    EOF = -23,
    /// 접근에러
    ACCESS = -24,
    /// 부적절한 핸들값
    INVALIDHANDLE = -25,
    /// 부적절한 System Operation
    INVALIDSYSOP = -26,
    NOTCHANGE = -27,
    /// 존재하지 않는 것
    NOTEXIST = -28,
    /// Lock이 해제됨
    UNLOCK = -29,
    /// Lock을 설정할 수 없는 리소스
    LOCK = -30,
    /// UID가 존재하지 않는 리소스
    HASNOUID = -31,
    /// Max값이 없음
    NOLIMIT = -32,
    /// 이미 정의된 이벤트
    ALREADYEXISTEVENT = -33,
    /// 플랫폼 종료
    PLTEXIT = -34,
    /// 메모리가 모자람
    INSUFSPACE = -35,
    /// 접근이 거부됨
    ACCESSDENY = -36,
    /// 같은 이름의 파일이 이미 존재함
    DUPNAME = -37,
    INVALIDSTATUS = -38,
    NORES = -39,
    PLOCK = -40,
    GLOCK = -41,
    INCORRECTPASSWORD = -42,
    INVALIDRESGROUP = -43,
    INVALIDTERMRES = -44,
    /// Annunciator에 사용자가 설정한 아이콘의 크기가 맞지 않는 경우
    NOTFITSIZE = -45,
    /// 그룹 lock을 지원하지 않음
    NOTSUPPORTGLOCK = -46,
    /// lock을 지원하지 않음
    NOTSUPPORTLOCK = -47,
    /// 캐릭벨 기능을 지원안함
    NOTSUPPORTCBELL = -48,
    /// 지원하지 않는 포맷임
    INVALIDFORMAT = -49,
    /// 단말이 지원하는 캐릭벨 착신번호마다 많은 매개 변수가 전달됨
    TOOMANYPARAM = -50,
    /// 이미지 디코딩시 Scale을 지원하지 않는 경우
    NOTSCALE = -51,
    /// 비정상적으로 네트워크 연결이 종료된 경우
    NETCLOSE = -52,
    /// 비정상적으로 소켓 연결이 종료된 경우
    SOCKETCLOSE = -53,
    /// 해당 프레임이 존재하지 않을 때
    NOFRAME = -54,
    /// 개별 락 지원하지 않음
    NOTSUPPORTPLOCK = -55,
    /// 디바이스가 활성화되지 않음
    NOTACTIVE = -56,
    /// WCDMA에서 CDMA로 모드 변경
    NETMODECHANGE = -57,
    /// 잘못된 볼륨 소스
    INVALIDSOURCE = -58,
    /// 이미지를 Resize 못하는 경우
    NOTRESIZE = -59,
    /// IO 장치가 close 된 경우
    DEVCLOSE = -60,
    /// OEM의 사정에 의해 WIPI 어플리케이션의 특정 동작이 중지되는 경우
    OEMERROR = -61,
    /// 해당 타입을 지원하지 않음
    NOTSUPPORTTYPE = -62,
    /// 찾을 수 없음
    NOTFOUND = -63,
    /// PROV Agent에서 데이터를 가져오지 못한 경우
    GETPROV = -64,
    /// 데이터가 잘못됨.
    INVALIDDATA = -65,
    /// 최대값을 벗어남.
    MAXCOUNT = -66,
    /// 장치가 장착되지 않은 경우
    NODEVICE = -67,
    /// LBS가 MS Assisted 방식으로 사용 중인 경우
    INUSE_BY_MSASSITED = -68,
    /// LBS가 MS Based 방식으로 사용 중인 경우
    INUSE_BY_MSBASED = -69,
    /// LBS가 Cell Based 방식으로 사용 중인 경우
    INUSE_BY_CELLBASED = -70,
    /// 삭제 불가능
    NODELETE = -71,
    /// 지원하지 않는 메소드
    NOTSUPPORTMETHOD = -72,
    /// MP3 파일 재생시 expire된 경우
    EXPIREDDATA = -73,
    /// Authentication Error
    AUTHENTICATE = -74,
    /// WCDMA에서 CDMA로 모드 변경이 완료되어 재접속 가능한 경우
    NETMODEREADY = -75,
    /// 압축 해제 실패
    UNCOMPRESS = -76,
    /// DCF Header 틀림
    BAD_DCF_INFORM = -77,
    /// 만료기한 지남
    DATE_EXPIRED = -78,
    /// 단말이 망에 등록되지 않았음
    DEVICE_NOT_REGISTERED = -79,
    /// DCF의 소유자와 단말의 MIN 번호가 다름
    INVALID_OWNERSHIP = -80,
    /// DCF가 단말기에서 실행할 수 있는 Domain이 아님
    BAD_DOMAIN = -81,
    /// WIPI 2.0 DLL 초기화 실패
    INIT = -82,
    /// 상태 변화가 있음.
    CHANGED = -83,
    /// 상태 변화가 없음.
    NOTCHANGED = -84,
    /// 상태 변화가 없음.
    INUSE_BY_OTHER_SVCID = -85,
    /// 상태 변화가 없음.
    NETDORMANT = -86,
    /// 네트워크의 PPP를 다른 APN, NAI를 이용하여 연 상태, 현재는 연결 요청 수락 안됨
    PREEMPTED = -87,
    /// No Service 상태
    NOSERVICE = -88,

    /// Unknown values are mapped to this enum value.
    /// Note that this field doesn't exist in WIPI and the choice of this ordinal is arbitrary.
    UnknownValue = i32::MIN,
}

impl From<WIPICErrorCode> for i32 {
    fn from(value: WIPICErrorCode) -> Self {
        value as Self
    }
}

impl From<i32> for WIPICErrorCode {
    fn from(value: i32) -> Self {
        if value >= (Self::SUCCESS as i32) && value <= (Self::NOSERVICE as i32) {
            // SAFETY: WipiErrorCode has CWord repr and is unit only.
            let x: Self = unsafe { mem::transmute(value) };
            x
        } else {
            Self::UnknownValue
        }
    }
}

impl TypeConverter<WIPICErrorCode> for WIPICErrorCode {
    fn to_rust(_context: &mut dyn WIPICContext, raw: WIPICWord) -> WIPICErrorCode {
        let v: i32 = bytemuck::cast(raw);
        v.into()
    }

    fn from_rust(_context: &mut dyn WIPICContext, rust: WIPICErrorCode) -> WIPICWord {
        let v: i32 = rust.into();
        bytemuck::cast(v)
    }
}
