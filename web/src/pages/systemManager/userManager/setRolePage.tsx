import {
  Form,
  FormInstance,
  Input,
  Modal,
  Notification,
  Select,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { useContext, useEffect, useRef } from 'react';
import React from 'react';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { getUserRole, setUerRoles } from '@/api/user';
import { getRoleList } from '@/api/role';

function SetRolePage(props: { id: number; visible; setVisible; callback }) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const [loading, setLoading] = React.useState(false);

  const t = useLocale(locale);

  const [roles, setRoles] = React.useState([]);

  function fetchData() {
    if (props.id !== undefined && props.visible) {
      setLoading(true);
      getRoleList().then((res) => {
        const { success, data } = res.data;
        if (success) {
          setRoles(
            data.map((role) => {
              return {
                label: role.role_name,
                value: role.id,
              };
            })
          );
        }
      });
      getUserRole(props.id).then((res) => {
        const { success, data } = res.data;
        if (success) {
          formRef.current.setFieldsValue({
            user_id: props.id,
            role: data,
          });
        }
      }).finally(()=>{
        setLoading(false);
      });
    }
  }

  useEffect(() => {
    fetchData();
  }, [props.id, props.visible]);

  //提交修改
  const handleSubmit = () => {
    setLoading(true);
    formRef.current.validate().then((values) => {
      setUerRoles(values).then((res) => {
        const { success, message } = res.data;
        if (success) {
          Notification.success({ content: message, duration: 1000 });
          props.setVisible(false);
          props.callback();
        }
        setLoading(false);
      });
    });
  };

  return (
    <Modal
      title={t['searchTable.columns.operations.setrole']}
      visible={props.visible}
      onOk={() => {
        handleSubmit();
      }}
      onCancel={() => {
        props.setVisible(false);
      }}
      autoFocus={false}
      focusLock={true}
      confirmLoading={loading}
    >
      <Form
        ref={formRef}
        style={{ width: '95%', marginTop: '6px' }}
        labelCol={{ span: lang === 'en-US' ? 7 : 6 }}
        wrapperCol={{ span: lang === 'en-US' ? 17 : 18 }}
      >
        <FormItem
          required
          label={t['searchTable.columns.id']}
          field={'user_id'}
          hidden
        >
          <Input placeholder="" />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.operations.setrole']}
          field={'role'}
        >
          <Select
            placeholder={t['searchForm.placeholder']}
            options={roles}
            mode="multiple"
            allowClear
          />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default SetRolePage;
