import {
  Form,
  FormInstance,
  Input,
  Modal,
  Notification,
  TreeSelect,
} from '@arco-design/web-react';
import locale from './locale';
import useLocale from '@/utils/useLocale';
import { GlobalContext } from '@/context';
import { useContext, useEffect, useRef } from 'react';
import React from 'react';
import FormItem from '@arco-design/web-react/es/Form/form-item';
import { getRoleResource, setRoleResource } from '@/api/role';
import { getResourceTree } from '@/api/resource';

function SetRoleResourcePage(props: {
  id: number;
  visible;
  setVisible;
  callback;
}) {
  const formRef = useRef<FormInstance>();

  const { lang } = useContext(GlobalContext);

  const [loading, setLoading] = React.useState(false);

  const t = useLocale(locale);

  const [resources, setResources] = React.useState([]);

  function fetchData() {
    if (props.id !== undefined && props.visible) {
      setLoading(true);
      getResourceTree().then((res) => {
        const { success, data } = res.data;
        if (success) {
          setResources(data);
        }
      });
      getRoleResource(props.id).then((res) => {
        const { success, data } = res.data;
        if (success) {
          formRef.current.setFieldsValue({
            role_id: props.id,
            resource: data,
          });
        }
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
      setRoleResource(values).then((res) => {
        const { success, message } = res.data;
        if (success) {
          Notification.success({ content: message, duration: 1000 });
          props.setVisible(false);
          props.callback();
        }
      }).finally(()=>{
        setLoading(false);
      });
    });
  };

  return (
    <Modal
      title={t['searchTable.columns.operations.set_role_resource']}
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
          field={'role_id'}
          hidden
        >
          <Input placeholder="" />
        </FormItem>
        <FormItem
          required
          label={t['searchTable.columns.resource']}
          field={'resource'}
        >
          <TreeSelect
            placeholder={t['searchTable.columns.operations.setrole']}
            treeData={resources}
            multiple
            allowClear
            treeCheckable
            loading={loading}
          />
        </FormItem>
      </Form>
    </Modal>
  );
}

export default SetRoleResourcePage;
