<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ตรวจสอบการแก้ไขประกาศข่าวสารสำเร็จ</name>
   <tag></tag>
   <elementGuidId>9f153cc8-cc5d-4008-8401-c286681a0cd4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${id_announcement}\&quot;,\n  \&quot;image\&quot;: \&quot;https://residentialfamily-dev.centralpattana.co.th/dev/assets/upload/ads/905c3775-059d-4d0e-a918-5865093364dc.jpg\&quot;,\n  \&quot;project\&quot;: {\n    \&quot;id\&quot;: \&quot;a56b8623-1b44-4da8-9a19-d4479981c8fa\&quot;\n  },\n  \&quot;title\&quot;: {\n    \&quot;th\&quot;: \&quot;ทดสอบแก้ไขหัวข้อประกาศ\&quot;,\n    \&quot;en\&quot;: \&quot;Test Edit Tiltle announcment\&quot;\n  },\n  \&quot;description\&quot;: {\n    \&quot;th\&quot;: \&quot;ทดสอบรายละเอียดแบบย่อ\&quot;,\n    \&quot;en\&quot;: \&quot;Test Detail\&quot;\n  },\n  \&quot;isHighlight\&quot;: false,\n  \&quot;publishedStartAt\&quot;: \&quot;2024-05-28T17:00:00.000Z\&quot;,\n  \&quot;publishedEndAt\&quot;: \&quot;2024-05-30T17:00:00.000Z\&quot;,\n  \&quot;content\&quot;: {\n    \&quot;th\&quot;: \&quot;\u003cp\u003eทดสอบรายละเอียด\u003c/p\u003e\&quot;,\n    \&quot;en\&quot;: \&quot;\u003cp\u003eTest Description\u003c/p\u003e\&quot;\n  },\n  \&quot;buttonType\&quot;: \&quot;url\&quot;,\n  \&quot;buttonName\&quot;: {\n    \&quot;th\&quot;: \&quot;ทดสอบเพิ่มชื่อปุ่ม\&quot;,\n    \&quot;en\&quot;: \&quot;Test Button name\&quot;\n  },\n  \&quot;buttonValue\&quot;: \&quot;Test button\&quot;,\n  \&quot;album\&quot;: [],\n  \&quot;tag\&quot;: \&quot;announcement\&quot;,\n  \&quot;publishedDateEnd\&quot;: null,\n  \&quot;highlightType\&quot;: \&quot;\&quot;,\n  \&quot;highlightStartAt\&quot;: null,\n  \&quot;highlightEndAt\&quot;: null,\n  \&quot;active\&quot;: false,\n  \&quot;album_status\&quot;: null,\n  \&quot;userEdit\&quot;: \&quot;ภัคศุภาวงค์ จั่นแพ แก้ไขเมื่อ 14/08/64\&quot;,\n  \&quot;projectShow\&quot;: [],\n  \&quot;group_list\&quot;: null,\n  \&quot;ads_show\&quot;: null,\n  \&quot;notification\&quot;: false,\n  \&quot;notificationScheduleId\&quot;: null,\n  \&quot;project_about\&quot;: null\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>560101c1-3843-437c-a59a-0f1ecd95922b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>0431e18c-f987-4056-b404-170aab3a8006</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project/a56b8623-1b44-4da8-9a19-d4479981c8fa/announcement</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>490c66bc-a5ba-4e0b-9f44-cf7bc387fcb9</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_announcement</defaultValue>
      <description></description>
      <id>c7b53901-4df5-4bea-bac7-5b0aee97d87b</id>
      <masked>false</masked>
      <name>id_announcement</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
